use crate::hooks_harness;
use std::sync::atomic::Ordering;
// use libafl::{
//     executors::ExitKind,
//     inputs::{BytesInput, HasTargetBytes},
//     Error,
// };
// use libafl_bolts::AsSlice;
// use libafl_qemu::{elf::EasyElf, ArchExtras, GuestAddr, GuestReg, MmapPerms, Qemu, Regs};
use std::cell::RefCell;
use std::collections::HashMap;
use crate::stream;
use libafl::{
    executors::ExitKind,
    inputs::{BytesInput, HasTargetBytes},
    Error,
};
use libafl_bolts::{os::unix_signals::Signal, AsSlice};
use libafl_qemu::{
    elf::EasyElf, ArchExtras, GuestAddr, GuestReg, MmapPerms, 
    Qemu, QemuExitReason, QemuShutdownCause, Regs
};
use std::process;



pub struct Harness {
    qemu: Qemu,
    input_addr: GuestAddr,
    pub pc: GuestAddr,
    pub stack_ptr: GuestAddr,
    ret_addr: GuestAddr,
}

pub const MAX_INPUT_SIZE: usize = 1_048_576; // 1MB

impl Harness {
    pub fn find_main_from_start(qemu: Qemu, entry_point: GuestAddr) -> Option<GuestAddr> {
        eprintln!("finding main from entry point: {:#x}", entry_point);
        let binary_path = qemu.binary_path();
        let file_bytes = std::fs::read(binary_path).ok()?;
        
        let e_type = u16::from_le_bytes(file_bytes.get(16..18)?.try_into().ok()?);
        let vaddr = entry_point - qemu.load_addr();
        let file_offset = if e_type == 2 {
            // eprintln!("non PIE");
            vaddr.saturating_sub(0x400000) as usize
        } else {
            // eprintln!("PIE");
            vaddr as usize
        };
        
        let buf = file_bytes.get(file_offset..file_offset + 64)?;
        // eprintln!("_start bytes: {:02x?}", &buf[..20]);
        
        for i in 0..buf.len().saturating_sub(6) {
            if buf[i] == 0x48 && buf[i+1] == 0xc7 && buf[i+2] == 0xc7 {
                let imm = u32::from_le_bytes(buf[i+3..i+7].try_into().ok()?);
                // eprintln!("found main at {:#x}", imm);
                return Some(imm as GuestAddr);
            }
        }
        None
    }
    /// Change environment
    #[inline]
    pub fn edit_env(_env: &mut Vec<(String, String)>) {}

    /// Change arguments
    #[inline]
    pub fn edit_args(_args: &mut Vec<String>) {}

    /// Helper function to find the function we want to fuzz.
    fn start_pc(qemu: Qemu) -> Result<GuestAddr, Error> {
        let mut elf_buffer = Vec::new();
        let elf = EasyElf::from_file(qemu.binary_path(), &mut elf_buffer)?;
        let is_static = elf.get_section(".interp", qemu.load_addr()).is_none();

        if let Some(start_pc) = elf.resolve_symbol("main", qemu.load_addr()){
                    return  Ok(start_pc);
                }
        // eprintln!("no main found so entry point is used");

        let entry_point = elf.entry_point(qemu.load_addr()).expect("no entry point");
        if let Some(main_addr) = Self::find_main_from_start(qemu, entry_point) {
            // eprintln!("found main dynamically: {:#x}", main_addr);
            return Ok(main_addr);
        }
        if let Some(range) = elf.get_section(".text", qemu.load_addr()) {
            // eprintln!("falling back to .text start: {:#x}", range.start);
            return Ok(range.start);
        }
        Ok(entry_point)
    }

    /// Initialize the emulator, run to the entrypoint (or jump there) and return the [`Harness`] struct
    pub fn init(qemu: Qemu) -> Result<Harness, Error> {
        let start_pc = Self::start_pc(qemu)?;
        log::info!("start_pc @ {start_pc:#x}");
       
        qemu.entry_break(start_pc);
       
        let mut elf_buffer = Vec::new();
        let elf = EasyElf::from_file(qemu.binary_path(), &mut elf_buffer)?;
        let elf_entry = elf.entry_point(qemu.load_addr()).expect("no entry point");
        if let Some(real_main) = Self::find_main_from_start(qemu, elf_entry) {
            // eprintln!("found real main at {:#x}, updating HARNESS_ENTRY", real_main);
            hooks_harness::HARNESS_ENTRY.store(real_main as u64, Ordering::Relaxed);
        }

        let ret_addr: GuestAddr = qemu
            .map_private(0, 4096, MmapPerms::ReadWrite)
            .map_err(|e| Error::unknown(format!("Failed to map fake ret: {e:}")))?;
            // .read_return_address()
            // .map_err(|e| Error::unknown(format!("Failed to read return address: {e:?}")))?;
        log::info!("ret_addr = {ret_addr:#x}");
        hooks_harness::HARNESS_FAKE_RET.store(ret_addr as u64, Ordering::Relaxed);
        
        qemu.set_breakpoint(ret_addr);
        
        let input_addr = qemu
            .map_private(0, MAX_INPUT_SIZE, MmapPerms::ReadWrite)
            .map_err(|e| Error::unknown(format!("Failed to map input buffer: {e:}")))?;
        
        let pc: GuestReg = qemu
            .read_reg(Regs::Pc)
            .map_err(|e| Error::unknown(format!("Failed to read PC: {e:?}")))?;

        let stack_ptr: GuestAddr = qemu
            .read_reg(Regs::Sp)
            .map_err(|e| Error::unknown(format!("Failed to read stack pointer: {e:?}")))?;

        let ret_addr: GuestAddr = qemu
            .read_return_address()
            .map_err(|e| Error::unknown(format!("Failed to read return address: {e:?}")))?;
       
        let argc: GuestReg = qemu.read_reg(Regs::Rdi).unwrap();
        let argv: GuestReg = qemu.read_reg(Regs::Rsi).unwrap();
        // eprintln!("at entry_break: argc={:#x} argv={:#x}", argc, argv);
        hooks_harness::HARNESS_PC.store(pc as u64, Ordering::Relaxed);
        hooks_harness::HARNESS_SP.store(stack_ptr as u64, Ordering::Relaxed);
        hooks_harness::HARNESS_ARGC.store(argc as u64, Ordering::Relaxed);
        hooks_harness::HARNESS_ARGV.store(argv as u64, Ordering::Relaxed);
        hooks_harness::HARNESS_READY.store(true, Ordering::Relaxed); 
        hooks_harness::SNAPSHOT_TAKEN.store(true, Ordering::Relaxed);
       
        Ok(Harness {
            qemu,
            input_addr,
            pc,
            stack_ptr,
            ret_addr,
        })
    }

    /// If we need to do extra work after forking, we can do that here.
    #[inline]
    #[expect(clippy::unused_self)]
    pub fn post_fork(&self) {}

    pub fn run(&self, input: &BytesInput) -> ExitKind {
        self.reset(input).unwrap();
        ExitKind::Ok
    }

    fn reset(&self, input: &BytesInput) -> Result<(), Error> {
        hooks_harness::FAKED_FD_MAPPING.with(|f| {
            let map = f.borrow();
            for (_fake_fd, real_fd) in map.iter() {
                unsafe { libc::close(*real_fd); }
            }
        });

        hooks_harness::FAKED_FD_MAPPING.with(|f: &RefCell<HashMap<i32,i32>>| {
            f.borrow_mut().clear();
        });

        hooks_harness::REAL_SOCKETS.with(|s| {
            let sockets = s.borrow();
            // eprintln!("[RESET] closing {} sockets in reset", sockets.len());
            for fd in sockets.iter() {
                unsafe { libc::close(*fd); }
            }
            drop(sockets);
            s.borrow_mut().clear();
        });

        let target = input.target_bytes();
        let mut buf = target.as_slice();
        let mut len = buf.len();
        if len > MAX_INPUT_SIZE {
            buf = &buf[0..MAX_INPUT_SIZE];
            len = MAX_INPUT_SIZE;
        }
        let len = len as GuestReg;

        stream::set_current_stream(buf);

        self.qemu.write_mem(self.input_addr, buf).map_err(|e| {
            Error::unknown(format!(
                "Failed to write to memory@{:#x}: {e:?}",
                self.input_addr
            ))
        })?;

        self.qemu
            .write_reg(Regs::Pc, self.pc)
            .map_err(|e| Error::unknown(format!("Failed to write PC: {e:?}")))?;

        self.qemu
            .write_reg(Regs::Sp, self.stack_ptr)
            .map_err(|e| Error::unknown(format!("Failed to write SP: {e:?}")))?;

        self.qemu
            .write_return_address(self.ret_addr)
            .map_err(|e| Error::unknown(format!("Failed to write return address: {e:?}")))?;

        // hooks_harness::set_harness_ret_addr(self.ret_addr);
        // moet deze twee hieronder nog ff checken:

        // self.qemu
        //     .write_function_argument(0, self.input_addr)
        //     .map_err(|e| Error::unknown(format!("Failed to write argument 0: {e:?}")))?;

        // self.qemu
        //     .write_function_argument(1, len)
        //     .map_err(|e| Error::unknown(format!("Failed to write argument 1: {e:?}")))?;

        self.qemu.write_reg(Regs::Rdi, hooks_harness::HARNESS_ARGC.load(Ordering::Relaxed) as GuestAddr).unwrap();
        self.qemu.write_reg(Regs::Rsi, hooks_harness::HARNESS_ARGV.load(Ordering::Relaxed) as GuestAddr).unwrap();
        
        unsafe {
            // let _ = self.qemu.run();
            match self.qemu.run() {
                    Ok(QemuExitReason::Breakpoint(_)) => {}
                    Ok(QemuExitReason::End(QemuShutdownCause::HostSignal(
                        Signal::SigInterrupt,
                    ))) => process::exit(0),
                    Ok(QemuExitReason::End(QemuShutdownCause::GuestReset)) => {}
                    _ => {}
                }
        };
        Ok(())
    }
}

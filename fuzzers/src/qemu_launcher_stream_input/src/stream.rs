// byte stream layout 
pub const STREAM_SIZE: usize = 800;

//access 
pub const OFFSET_ACCESS: usize = 0;
pub const SIZE_ACCESS: usize = 4;

//FSTAT
pub const OFFSET_FSTAT: usize = 4;
pub const SIZE_FSTAT: usize = 144;

//OPENAT
pub const OFFSET_OPENAT: usize = 148;
pub const SIZE_OPENAT: usize = 4;

//PRLIMIT64
pub const OFFSET_PRLIMIT64: usize = 152;
pub const SIZE_PRLIMIT64: usize = 4;

//READ
pub const OFFSET_READ: usize = 156;
pub const SIZE_READ: usize = 256;

//PREAD64
pub const OFFSET_PREAD64: usize = 412;
pub const SIZE_PREAD64: usize = 256;


pub fn set_seed(path: &str){
    let mut buf = vec![0u8; STREAM_SIZE]; // init with zero's

    let access: i32 = 0;
    buf[OFFSET_ACCESS..OFFSET_ACCESS + SIZE_ACCESS].copy_from_slice(&access.to_le_bytes());
    std::fs::write(path,buf).expect("cannot writeeee\n");
    println!("printed seeed");
}

use std::cell::RefCell;
thread_local!{
    // pub static CURRENT_STREAM: RefCell<> = RefCell::new(EnvVector::default_vec());
    pub static CURRENT_STREAM: RefCell<Vec<u8>> = RefCell::new(vec![0u8; STREAM_SIZE]);
}

pub fn set_current_stream(bytes: &[u8]){
    CURRENT_STREAM.with(|b: &RefCell<Vec<u8>>| {
        let mut b = b.borrow_mut();
        b.clear();
        b.extend_from_slice(bytes);
    });
}

pub fn get_current_bytes(offset: usize, size: usize) -> Vec<u8>{

    CURRENT_STREAM.with(|b: &RefCell<Vec<u8>>| {
        let b = b.borrow();
        b[offset..offset + size].to_vec()
    })

}
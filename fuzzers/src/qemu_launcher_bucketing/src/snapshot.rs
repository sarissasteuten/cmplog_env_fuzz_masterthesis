use core::sync::atomic::{AtomicBool, Ordering};

static SNAPSHOT_TRIGGERED: AtomicBool = AtomicBool::new(false);
static SNAPSHOT_TAKEN: AtomicBool = AtomicBool::new(false);

#[inline]
pub fn reset_snapshot_state() {
    SNAPSHOT_TRIGGERED.store(false, Ordering::Relaxed);
    SNAPSHOT_TAKEN.store(false, Ordering::Relaxed);
}

#[inline]
pub fn trigger_snapshot_on_first_interesting_syscall() {
    if !SNAPSHOT_TAKEN.load(Ordering::Relaxed) {
        eprintln!("SNAPSHOT_TRIGGERED");
        SNAPSHOT_TRIGGERED.store(true, Ordering::Relaxed);
    }
}

#[inline]
pub fn consume_trigger() -> bool {
    SNAPSHOT_TRIGGERED.swap(false, Ordering::AcqRel)
}

#[inline]
pub fn mark_taken_once() -> bool {
    !SNAPSHOT_TAKEN.swap(true, Ordering::AcqRel)
}

#[inline]
pub fn is_taken() -> bool {
    SNAPSHOT_TAKEN.load(Ordering::Acquire)
}
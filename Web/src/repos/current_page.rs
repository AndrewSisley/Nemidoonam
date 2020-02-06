use std::sync::atomic::{AtomicI32, Ordering};

static CURRENT_PAGE_ID: AtomicI32 = AtomicI32::new(0);

pub fn get() -> i32 {
    return CURRENT_PAGE_ID.load(Ordering::Relaxed);
}

pub fn set(id: i32) {
    CURRENT_PAGE_ID.store(id, Ordering::Relaxed);
}

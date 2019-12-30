use std::sync::atomic::{AtomicI32, Ordering};

static CURRENT_LANGUAGE_ID: AtomicI32 = AtomicI32::new(1);

pub fn get() -> i32 {
    return CURRENT_LANGUAGE_ID.load(Ordering::Relaxed);
}

pub fn set(id: i32) {
    CURRENT_LANGUAGE_ID.store(id, Ordering::Relaxed);
}

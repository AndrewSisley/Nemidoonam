use std::sync::atomic::{AtomicI32, Ordering};

static TARGET_LANGUAGE_ID: AtomicI32 = AtomicI32::new(2);

pub fn get() -> i32 {
    return TARGET_LANGUAGE_ID.load(Ordering::Relaxed);
}

pub fn set(id: i32) {
    TARGET_LANGUAGE_ID.store(id, Ordering::Relaxed);
}

use std::sync::atomic::{AtomicU64, Ordering};

static UI_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UiId(u64);

impl UiId {
    pub fn new() -> Self {
        Self(UI_ID.fetch_add(1, Ordering::SeqCst))
    }

    pub fn new_none() -> Self {
        Self(0)
    }
}

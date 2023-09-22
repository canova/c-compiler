use std::sync::atomic::{AtomicUsize, Ordering};

static LABEL_COUNTER: AtomicUsize = AtomicUsize::new(0);

pub fn unique_label() -> String {
    format!("L{}", LABEL_COUNTER.fetch_add(1, Ordering::SeqCst))
}

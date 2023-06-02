// oneshot -> sending one message from one thread to another

// Unsafecell -> for storage
// Atomic Bool -> state

use std::sync::atomic::AtomicBool;

pub struct Channel<T> {
    queue: Option<T>,
    ready: AtomicBool,
}

impl<T> Channel<T> {}

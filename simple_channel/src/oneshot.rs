// oneshot -> sending one message from one thread to another

// Unsafecell -> for storage
// Atomic Bool -> state

use std::{ sync::atomic::AtomicBool, cell::UnsafeCell, mem::MaybeUninit };

use std::sync::atomic::Ordering::{ Release, Acquire };

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>, //UnsafeCell<Option<T>>,
    ready: AtomicBool,
}

// channel is safe to share if T is a send type

unsafe impl<T> Sync for Channel<T> where T: Send {} //trait

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    /// Safety: Only call this once!
    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);
        self.ready.store(true, Release)
    }

    pub fn is_read(&self) -> bool {
        self.ready.load(Acquire)
    }

    /// Safety: only call this once;
    /// and only after is_read() returns True!
    ///
    pub unsafe fn receive(&self) -> T {
        (*self.message.get()).assume_init_read()
    }
}

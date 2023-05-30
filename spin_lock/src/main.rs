use std::cell::UnsafeCell;
use std::thread;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{ Acquire, Release };

use std::ops::{ Deref, DerefMut };

// why we have used acquire and release here as ordering and not relaxed

// unlock() will have a happen-before relationship with lock()

// We will provide a data change or exclusive reference in the lock state, so that we can change the locked data

// we want to return `&mut T` reference from Lock()
pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

// for guard behave like an exclusive refernce, implement the special Deref and DerefMut

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> Deref for Guard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.lock.value.get() }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        self.lock.locked.store(false, Release)
    }
}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock<'a>(&'a self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            // hint may be at compile time
            // the hint will tell the processor that we are spinning while waiting for something to change
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }
    /// Safety: The &mut T fromo lock () must be gone!
    /// (And no cheating by keeping reference to fields of that T around)

    pub fn unlock(&self) {
        self.locked.store(false, Release)
    }
}

fn main() {
    let x = SpinLock::new(Vec::new());

    thread::scope(|s| {
        s.spawn(|| x.lock().push(1));

        s.spawn(|| {
            let mut g = x.lock();
            g.push(2);
            g.push(2);
        })
    });

    let g = x.lock();

    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);

    println!("Hello, world!");
}

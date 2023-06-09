use std::sync::atomic::Ordering::{ Acquire, Release };
use std::sync::atomic::{ AtomicU64, AtomicBool };
use std::sync::atomic::Ordering::Relaxed;

use std::thread;
use std::time::Duration;

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

pub fn example() {
    thread::spawn(|| {
        DATA.store(123, Relaxed);
        READY.store(true, Release); // Everything from before this store ..
    });
    while !READY.load(Acquire) {
        // .. is visible after this loads `true`.
        thread::sleep(Duration::from_millis(1000));
        println!("waiting...");
    }
    println!("{}", DATA.load(Relaxed));
}

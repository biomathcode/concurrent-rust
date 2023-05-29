use std::thread;

use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

use chap_3::example;

static X: AtomicU32 = AtomicU32::new(0);

fn a() {
    X.fetch_add(5, Relaxed);
    X.fetch_add(5, Relaxed);
}

fn b() {
    let x = X.load(Relaxed);

    println!("x value : {}", x)
}

fn main() {
    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(|| {
                a();
                b();
            });
        }
    });

    b();
    example();

    println!("Hello, world!");
}

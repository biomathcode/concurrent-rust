use std::sync::Arc;
use std::thread;

pub fn arc_fn() {
    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    thread::spawn(move || dbg!(a));
    thread::spawn(move || dbg!(b));
}

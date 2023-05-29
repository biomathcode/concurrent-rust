use core::num;
use std::{ thread, vec, ops::Add };

use chap_1::arc_fn;

fn main() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    let num = vec![1, 2, 3, 4, 5];

    println!("this is the sum {} {}", sum(1, 20), sum(0.352, 0.2352));

    thread
        ::spawn(move || {
            for n in num {
                println!("{n}");
            }
        })
        .join()
        .unwrap();

    println!("Hello, world! from main thread");

    arc_fn();

    t1.join().unwrap();
    t2.join().unwrap();
}

fn f() {
    println!("Hello from another thread.");

    let id = thread::current().id();
    println!("this is my thread id: {id:?}");
}

fn sum<T>(num1: T, num2: T) -> T where T: Add<Output = T> {
    num1 + num2
}

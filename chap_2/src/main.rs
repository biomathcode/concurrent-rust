use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;
use std::thread;

fn main() {
    thread::scope(|s| {
        for t in 0..4 {
            s.spawn(|| {
                let id = thread::current().id();

                let num = allocate_new_id();

                println!("length: {}, {} ", num, 's');
            });
        }
    });
}

fn allocate_new_id() -> u32 {
    static NEXT_ID: AtomicU32 = AtomicU32::new(0);
    let mut id = NEXT_ID.load(Relaxed);
    loop {
        assert!(id < 1000, "TOO MANY IDS");
        match NEXT_ID.compare_exchange_weak(id, id + 1, Relaxed, Relaxed) {
            Ok(_) => {
                return id;
            }
            Err(v) => {
                id = v;
            }
        }
    }
}

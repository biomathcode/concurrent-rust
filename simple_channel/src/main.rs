use std::{ sync::{ Mutex, Condvar }, collections::VecDeque };

mod oneshot;

// CondVar will block a thread -> No CPU time -> Waiting! -> event

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    ready: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self { queue: Mutex::new(VecDeque::new()), ready: Condvar::new() }
    }
    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message); // sending message here
        self.ready.notify_one(); // -> notify the one thread that is waiting for the response
    }

    pub fn receive(&self) -> T {
        // we will be blocking the thread untill we get a message
        let mut b = self.queue.lock().unwrap();

        loop {
            if let Some(message) = b.pop_back() {
                return message;
            }
            b = self.ready.wait(b).unwrap(); // block the thread untill we get a notification
        }
    }
}

fn main() {
    println!("Hello, world!");
}

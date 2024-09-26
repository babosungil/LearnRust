use core::panic;
use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicBool;
use std::sync::mpsc::channel;
use std::sync::{Condvar, Mutex, Arc};
use std::collections::VecDeque;
use std::sync::atomic::Ordering::{Release, Acquire, Relaxed};
use std::thread;
use std::thread::scope;

pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T>{
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    pub fn receive(&self) -> T {
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(message) = b.pop_front() {
                return message;
            }
            b = self.item_ready.wait(b).unwrap();
        }
    }
}






pub struct Sender<T> {
    channel: Arc<Channel2<T>>,    
}

impl<T> Sender<T> {
    pub fn send(self, message: T) {
        unsafe { (*self.channel.message.get()).write(message) };
        self.channel.ready.store(true, Release);
    }
}

pub struct Receiver<T> {
    channel: Arc<Channel2<T>>,
}

impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        self.channel.ready.load(Relaxed)
    }

    pub fn receive(self) -> T {
        if !self.channel.ready.swap(false, Acquire) {
            panic!("no message available!");
        }
        unsafe { (*self.channel.message.get()).assume_init_read() }
    }
}

struct Channel2<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    //in_use: AtomicBool,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T : Send {

}

impl<T> Channel2<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            //in_use: AtomicBool::new(false),
            ready: AtomicBool::new(false),
        }
    }

    /*
    pub unsafe fn send(&self, message: T) {
        if self.in_use.swap(true, Relaxed) {
            panic!("cannot send more than one message!");
        }
        (*self.message.get()).write(message);
        self.ready.store(true, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Relaxed)
    }

    pub unsafe fn receive(&self) -> T {
        if !self.ready.load(Acquire) {
            panic!("no message available");
        }
        unsafe { (*self.message.get()).assume_init_read() }
    }
    */
}

impl<T> Drop for Channel2<T> {
    fn drop(&mut self) {
        if *self.ready.get_mut() {
            unsafe { self.message.get_mut().assume_init_drop() }
        }
    }
}

pub fn channel2<T>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(Channel2 {
        message: UnsafeCell::new(MaybeUninit::uninit()),
        ready: AtomicBool::new(false),
    });
    ( Sender { channel: a.clone() }, Receiver { channel: a } );
}


fn main() {
    /*
    let channel2 = Channel2::new();
    let t = thread::current();
    scope(|s: &thread::Scope<'_, '_>| {
        s.spawn(|| {
            channel2.send("message 1");
            t.unpark();
        });

        while !channel2.is_ready() {
            thread::park();
        }

        println!("{}", channel2.receive());
    });
    */

    /*
    scope(|s| {
        let (sender, receiver) = channel2();
        let t = thread::current();
        s.spawn(move || {
            sender.send("aaaaa");
            t.unpark();
        });
    });
    */
}

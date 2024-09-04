use std::time::Duration;
use std::{cell::Cell, marker::PhantomData, sync::Mutex};
use std::thread::{self, scope};

struct MyData {
    id : i32,
    _not_sync: PhantomData<Cell<()>>,   // sync : x, send : 0

    pointer: *mut i32,                  // unknow compiler for this member (send sync)
}

unsafe impl Send for MyData {}          // for pointer
unsafe impl Sync for MyData {}          // for pointer

fn main() {
    let mutex = Mutex::new(0);
    scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut mutex_guard = mutex.lock().unwrap();
                for _ in 0..1000 {
                    *mutex_guard += 1;
                }                
            });
            thread::sleep(Duration::from_millis(1));
            println!("{}", mutex.lock().unwrap());
        }        
    });
}


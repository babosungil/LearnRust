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
            println!("test 1 - {}", mutex.lock().unwrap());
        }
    });


    
    let mut vec_test = Vec::new();
    vec_test.push(1);
    let mutex = Mutex::new(vec_test);
    scope(|s|{
        for _ in 0..10 {
            s.spawn(|| {

                for _ in 0..10000 {
                    mutex.lock().unwrap().push(1);
                }
                println!("test 2 - loop fin");
            });

            thread::sleep(Duration::from_millis(1));
            println!("test 2 - {}", mutex.lock().unwrap().len());
        }
    });


    /*
    scope(|s|{
        println!("test 3 start");
        for i in 0..10 {
            println!("test 3 start id : {i}");
            s.spawn(|| {
                for _ in 0..10000 {
                    if let a = mutex.lock().unwrap().len() > 1 {
                        mutex.lock().unwrap().pop();                        // not release mutex guard
                    }
                }
                println!("test 3 - loop fin");
            });

            thread::sleep(Duration::from_millis(1));
            println!("test 3 - {}", mutex.lock().unwrap().len());
        }
    });
    */


    scope(|s|{
        for i in 0..3 {
            println!("test 4 start id : {i}");
            let thread = s.spawn(|| {                
                thread::park();
                mutex.lock().unwrap().push(1);
            });
            thread.thread().unpark();

            println!("test 4 - {}", mutex.lock().unwrap().len());
        }
    });
}


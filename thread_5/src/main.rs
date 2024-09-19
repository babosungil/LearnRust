use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use std::thread;
use std::time::Duration;

/*
static one: AtomicI32 = AtomicI32::new(0);
static two: AtomicI32 = AtomicI32::new(0);

fn func_a() {
    one.fetch_add(1, Ordering::Relaxed);
    two.fetch_add(10000, Ordering::Relaxed);
}

fn func_b() {
    let num_one = one.load(Ordering::Relaxed);
    let num_two = two.load(Ordering::Relaxed);

    println!("{}, {}", num_one, num_two);
}

fn func_c() {
    one.fetch_add(10000, Ordering::Relaxed);
    two.fetch_add(1, Ordering::Relaxed);
}
*/

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

fn main() {

    /*
    static count: i32 = 1000;

    let thread_a = thread::spawn(||{        
        for i in 0 .. count {
            func_a();
            thread::sleep(Duration::from_millis(1));
        }
    });
    let thread_b = thread::spawn(||{        
        for i in 0 .. count {
            func_b();
            thread::sleep(Duration::from_millis(1));
        }
    });
    let thread_c = thread::spawn(||{        
        for i in 0 .. count {
            func_c();
            thread::sleep(Duration::from_millis(1));
        }
    });

    thread_a.join().unwrap();
    thread_b.join().unwrap();
    thread_c.join().unwrap();
    */

    let thread_a = thread::spawn(|| {
        A.store(true, Ordering::SeqCst);
        if !B.load(Ordering::SeqCst) {
            unsafe { S.push('A'); }
        }
    });

    let thread_b = thread::spawn(|| {
        B.store(true, Ordering::SeqCst);
        if !A.load(Ordering::SeqCst) {
            unsafe { S.push('B'); }
        }
    });

    thread_a.join().unwrap();
    thread_b.join().unwrap();

    unsafe {
        println!("S : {:?}", S);
    }

    println!("fin!");
}

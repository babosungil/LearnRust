use std::os::windows::thread;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::AtomicI8;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::thread::scope;
use std::time::Duration;
use std::thread::sleep;
use core::sync::atomic::Ordering::Relaxed;


fn process(id : i32) {
    sleep(Duration::from_millis(100));
}

fn main() {
    let a = AtomicI8::new(100);
    let b = a.fetch_add(10, Ordering::Relaxed);
    let c = a.load(Ordering::Relaxed);
    println!("b: {b}, c: {c}");

    let b = a.fetch_add(100, Ordering::Relaxed);        // overflow wrapping
    let c = a.load(Ordering::Relaxed);
    println!("b: {b}, c: {c}");


    let checker = &AtomicUsize::new(0);
    scope(|s|{
        for i in 0..4 {
            s.spawn(move|| {
                for k in 0..25 {
                    process(i * 25 + k);
                    checker.fetch_add(1, Relaxed);
                }
            });
        }
    });

    loop {
        let n = checker.load(Relaxed);
        if n == 100 {
            break;
        }

        println!("working.. {n}/100 done");
        sleep(Duration::from_secs(1));
    }

    println!("fin!.");
}

use std::os::windows::thread;
use std::sync::atomic::{AtomicI32, AtomicI8, AtomicU64, AtomicUsize, Ordering};
use std::thread::scope;
use std::time::Duration;
use std::thread::sleep;
use core::sync::atomic::Ordering::Relaxed;
use std::time::Instant;


fn process(id : i32) -> u64 {
    let mut counter = 0;
    for _ in 0 .. id * 1000000 {
        counter += 1;
    }
    return counter;
}

fn main() {
    let a = AtomicI8::new(100);
    let b = a.fetch_add(10, Ordering::Relaxed);
    let c = a.load(Ordering::Relaxed);
    println!("b: {b}, c: {c}");

    let b = a.fetch_add(100, Ordering::Relaxed);        // overflow wrapping
    let c = a.load(Ordering::Relaxed);
    println!("b: {b}, c: {c}");


    
    println!("=== start! ===");

    let checker = &AtomicUsize::new(0);
    let time_total = &AtomicU64::new(0);
    let time_max = &AtomicU64::new(0);

    scope(|s|{
        for i in 0..4 {
            s.spawn(move|| {
                for k in 0..25 {
                    let time_start = Instant::now();
                    process(i * 25 + k);
                    checker.fetch_add(1, Relaxed);
                    let time_taken = time_start.elapsed().as_micros() as u64;

                    time_total.fetch_add(time_taken, Relaxed);
                    time_max.fetch_max(time_taken, Relaxed);
                }
            });
        }

        loop {
            let n: usize = checker.load(Relaxed);
            let total = Duration::from_micros(time_total.load(Relaxed));
            let max = Duration::from_micros(time_total.load(Relaxed));
            let mut avg = total;
            if n > 0 {
                avg = avg / n as u32;
            }

            println!("working.. {n}/100 done... average : {:?}, peak : {:?}", avg, max);
            if n == 100 {
                break;
            }
            sleep(Duration::from_millis(100));
        }
    });

    println!("=== fin!. ===");


    
    let checker_count = checker.load(Relaxed);
    match checker.compare_exchange(checker_count, 200, Relaxed, Relaxed) {
        Ok(_) => {
            println!("checker_count compare success - 100");
        },
        Err(v) => {
            println!("checker_count compare fail {v}");
        }
    }
    let checker_count = checker.load(Relaxed);
    println!("checker_count : {checker_count}");
}

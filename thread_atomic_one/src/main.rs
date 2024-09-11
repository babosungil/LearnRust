use std::sync::atomic::AtomicU32;
use std::{sync::atomic::AtomicBool, thread, time::Duration};
use std::sync::atomic::Ordering::Relaxed;

fn process_one() {
    println!("func : process_one - start");

    for i in 0..3 {
        thread::sleep(Duration::from_secs(1));
        //println!("func : process_one loop : {i}");
    }

    println!("func : process_one - end");
}

fn main() {
    static ATOMIC_BOOL: AtomicBool = AtomicBool::new(true);
    static ATOMIC_UINT: AtomicU32 = AtomicU32::new(0);
    static ATOMIC_SWITCH: AtomicBool = AtomicBool::new(false);

    let thj_work: thread::JoinHandle<()> = thread::spawn(||{
        while ATOMIC_BOOL.load(Relaxed) {
            process_one();
        }
    });

    let thj_counter_one: thread::JoinHandle<()> = thread::spawn(||{
        let mut count = 0;
        while ATOMIC_BOOL.load(Relaxed) {
            count += 1;
            ATOMIC_UINT.store(count, Relaxed);
            if count > 9999999 {
                count = 0;
            }

            thread::sleep(Duration::from_millis(500));
        }
    });
    
    
    let thj_pingpong_one = thread::spawn(||{
        thread::park();
        loop {
            if ATOMIC_BOOL.load(Relaxed) == false {
                break;
            }

            println!("pingpong one - loop");            
            thread::sleep(Duration::from_secs(1));

            if ATOMIC_SWITCH.load(Relaxed) {
                thread::park();
                ATOMIC_SWITCH.store(false, Relaxed);
            }
        }
    });


    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "q" => {
                println!("destroy process!");
                ATOMIC_BOOL.store(false, Relaxed);
                thj_pingpong_one.thread().unpark();
                break;
            },
            "w" => {
                println!("current count : {}", ATOMIC_UINT.load(Relaxed));
            },
            "s" => {
                ATOMIC_BOOL.store(false, Relaxed);
                println!("stop counter!");
            },
            "a" => {
                ATOMIC_BOOL.store(true, Relaxed);
                println!("restart counter!");
            }, 
            "z" => {
                ATOMIC_SWITCH.store(true, Relaxed);
            },
            "x" => {
                thj_pingpong_one.thread().unpark();
            },
            _ => println!("unkown command"),
        }
    }

    ATOMIC_BOOL.store(false, Relaxed);
    thj_work.join().unwrap();
    thj_counter_one.join().unwrap();
    thj_pingpong_one.join().unwrap();

    println!("fin!");
}

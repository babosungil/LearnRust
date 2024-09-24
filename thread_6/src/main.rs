use std::{sync::atomic::AtomicBool, thread, time::Duration, sync::atomic::Ordering};
use std::sync::atomic::fence;

const COUNT: usize = 10;
static mut DATA: [u64; COUNT] = [0; COUNT];
const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; COUNT] = [ATOMIC_FALSE; COUNT];

fn test_calc(index: usize) -> u64 {
    /*
    let mut temp = 0;
    for i in 0 .. index * 10000 {
        temp += i;
    }
    */
    (index + 1).try_into().unwrap()
}

fn main() {
    for i in 0..COUNT {
        thread::spawn(move || {
            let data = test_calc(i);
            unsafe { DATA[i] = data; }
            READY[i].store(true, Ordering::Release);
        });
    }

    thread::sleep(Duration::from_millis(500));

    let ready: [bool; COUNT] = std::array::from_fn(|i| READY[i].load(Ordering::Relaxed));
    if ready.contains(&true) {
        fence(Ordering::Acquire);
        for i in 0..10 {
            if ready[i] {
                println!("data{i} = {}", unsafe { DATA[i] });
            }
        }
    }
}

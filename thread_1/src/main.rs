use std::{thread::{self, JoinHandle}, time::Duration, vec};

fn main() {
    
    let mut handles = Vec::new();

    for i in 0..10 {
        let handle = thread::spawn(move || {
            println!("spawn thread : {}", i); 
        });
        //handle.join();
        handles.push(handle);

        //thread::sleep(Duration::from_millis(100));
    };
    for h in handles {
        let _ = h.join().unwrap();
    }    

    thread::spawn(|| {
        test_func(1);
    });    

    thread::spawn(|| {      
        test_func(2);
    });

    thread::sleep(Duration::from_millis(1));
}

fn test_func(param_1 : i32) {
    println!("call test_func with param {param_1}");
}
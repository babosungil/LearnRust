use std::{sync::{mpsc, Mutex}, thread, time::Duration};
use std::sync::Arc;

fn main() {

    ///////////////////////////////////////////////////////////////////
    // mutex + Arc
    let mutex = Mutex::new(1);
    {
        let mut num = mutex.lock().unwrap();        
        *num += 1;
        println!("mutex mun : {}", *num);
    }

    
    let rc_mutex = Arc::new(Mutex::new(1));
    let mut handles = vec![];

    for _ in 0..10 {
        let rc_mutex = Arc::clone(&rc_mutex);
        let handle = thread::spawn(move || {
            let mut temp = rc_mutex.lock().unwrap();
            *temp += 1;
            println!("{:?} {}", thread::current().id(), *temp);
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("fin all thread - {}", *rc_mutex.lock().unwrap());
    



    ///////////////////////////////////////////////////////////////////
    // channel
    let  (sender, receiver) = mpsc::channel();

    let sender_1 = sender.clone();
    thread::spawn(move || {
        let str = String::from("thread 1 msg 1");
        sender_1.send(str.clone()).unwrap();
        println!("log - {:?} ... str : {}", thread::current().id(), str);
    });

    let sender_2 = sender.clone();
    thread::spawn(move || {
        let str = vec![
            String::from("thread 2 msg 1"),
            String::from("thread 2 msg 2"),
            String::from("thread 2 msg 3"),
            String::from("thread 2 msg 4"),
            String::from("thread 2 msg 5"),
        ];

        for val in str {
            sender_2.send(val.clone()).unwrap();
            println!("log - {:?} ... str : {}", thread::current().id(), val);
            thread::sleep(Duration::from_millis(10));
        }
    });
    
    
    for received in receiver {
        println!("received : {}", received);
    }
    
}

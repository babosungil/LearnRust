
enum MyList {
    Cons(i32, Box<MyList>),
    Nil,
}

enum MyRcList {
    ConsRc(i32, Rc<MyRcList>),
    NilRc,
}

use crate::MyList::{Cons, Nil};
use crate::MyRcList::{ConsRc, NilRc};
use std::rc::Rc;

// reference counting
//Rc<T>

fn main() {
    // box new 
    let a = Cons(1, Box::new(Cons(11, Box::new(Nil))));
    let b = Cons(2, Box::new(a));
    //let c = Cons(3, Box::new(a));   // error - value moved prev line

    // rc new, clone
    let aa = Rc::new(ConsRc(4, Rc::new(ConsRc(11, Rc::new(NilRc)))));
    println!("1 reference count : {}", Rc::strong_count(&aa));
    let bb = ConsRc(12, Rc::clone(&aa));
    println!("2 reference count : {}", Rc::strong_count(&aa));
    {
        let cc = ConsRc(13, Rc::clone(&aa));
        println!("3 reference count : {}", Rc::strong_count(&aa));
    }
    println!("4 reference count : {}", Rc::strong_count(&aa));



    let aaa = Rc::new(ConsRc(4, Rc::new(ConsRc(11, Rc::new(NilRc)))));
    println!("1 weak count : {}", Rc::weak_count(&aa));
    let bbb = ConsRc(12, Rc::clone(&aa));
    println!("2 weak count : {}", Rc::weak_count(&aa));    
}

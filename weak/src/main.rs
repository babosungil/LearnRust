use std::cell::RefCell;
use std::rc::{Rc, Weak};

use MyList::{MyCons, Nil};

#[derive(Debug)]
enum MyList {
    MyCons(i32, RefCell<Rc<MyList>>),
    Nil,
}

impl MyList {
    fn tail(&self) -> Option<&RefCell<Rc<MyList>>> {
        match self {
            MyCons(_, item) => Some(item),
            Nil => None,
        }
    }
}


//use MyWeak::{My}

#[derive(Debug)]
struct MyData {
    val : i32,
    parent : RefCell<Weak<MyData>>,
    child : RefCell<Vec<Rc<MyData>>>,
}




fn main() {
    let rc_1 = Rc::new(MyCons(5, RefCell::new(Rc::new(Nil))));
    println!("rc_1 - initial rc count : {}", Rc::strong_count(&rc_1));
    println!("rc_1 - next item : {:?}", rc_1.tail());

    let rc_2 = Rc::new(MyCons(10, RefCell::new(Rc::clone(&rc_1))));
    println!("rc_1 - clone... rc count : {}", Rc::strong_count(&rc_1));
    println!("rc_2 - initial rc count : {}", Rc::strong_count(&rc_2));
    println!("rc_2 - next item : {:?}", rc_2.tail());

    if let Some(link) = rc_1.tail() {
        *link.borrow_mut() = Rc::clone(&rc_2);
        println!("rc_2 - clone")
    }

    println!("rc_1 - rc count : {}", Rc::strong_count(&rc_1));
    println!("rc_2 - rc count : {}", Rc::strong_count(&rc_2));

    //println!("rc_1 - {:?}", rc_1);            // check circular reference
    //println!("rc_1 - {:?}", rc_1.tail());     // check circular reference
    //println!("rc_2 - {:?}", rc_2);


    let my_data = Rc::new(MyData {
        val : 111,
        parent : RefCell::new(Weak::new()),
        child : RefCell::new(vec![]),
    });
    println!("my_data parent = {:?}", my_data.parent.borrow().upgrade());
    let parent_data = Rc::new(MyData {
        val : 222,
        parent : RefCell::new(Weak::new()),
        child : RefCell::new(vec![Rc::clone(&my_data)])
    });
    *my_data.parent.borrow_mut() = Rc::downgrade(&parent_data);
    println!("my_data parent = {:?}", my_data.parent.borrow().upgrade());   // no circular reference
}

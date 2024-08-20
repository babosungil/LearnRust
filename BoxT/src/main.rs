use std::ops::Deref;

enum List {
    //Cons(i32, List),    // error - recursive type `List` has infinite size
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)        
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<str> Drop for MyBox<str> {
    fn drop(&mut self) {
        println!("MyBox - drop");
    }
}


fn str_print(text : &str) {
    println!("str_print : {text}");
}


fn main() {
    let new_value = 1;                   // alloc in stack
    let box_value = Box::new(2);    // alloc in heap
    let ref_value = &new_value;    

    println!("new_value : {new_value}");
    println!("box_value : {box_value}");
    println!("ref_value : {}", *ref_value);

    //assert_eq!(1, ref_value);                 // error! - `{integer} == &{integer}`
    assert_eq!(1, *ref_value);


    let list = Cons(1, Box::new(
        Cons(2, Box::new(
            Cons(3, Box::new(Nil))
        ))
    ));
    
    let my_box = MyBox::new(32);
    //assert_eq!(32, my_box);       // error!
    assert_eq!(32, *my_box);        // use deref()
    //println!("my_box : {:?}", *my_box);
    drop(my_box);
    
    {
        let my_box_str = MyBox::new(String::from("my box str"));
        str_print(&my_box_str);
    }

    let my_box_str_2 = MyBox::new(String::from("my box str 2"));
    str_print(&my_box_str_2);

    println!("end of code");

}

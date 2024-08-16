
enum List {
    //Cons(i32, List),    // error - recursive type `List` has infinite size
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let new_value = 1;                   // alloc in stack
    let box_value = Box::new(2);    // alloc in heap

    let list = Cons(1, Box::new(
        Cons(2, Box::new(
            Cons(3, Box::new(Nil))
        ))
    ));
}

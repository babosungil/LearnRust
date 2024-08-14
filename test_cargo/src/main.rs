/// My Test Function
/// 
/// # HowTO !
/// let result = test(2)
/// 
/// # Panics
/// Panics Example!
/// 
/// # Errors
/// This is Error 
/// 
/// # Safety
/// safety!?
/// 

use babo_lib_1;

pub mod TeamColor {
    pub enum Colors {
        Red,
        Green,
        Blue,
    }
}

pub fn test(a : i32) -> i32 {
    a * a
}


fn main() {
    let value = babo_lib_1::add(1, 2);
    println!("babo_lib_1::add : {value}");

    let value = babo_lib_2::minus(4,3);
    println!("babo_lib_2::minus : {value}");

    let value = babo_lib_3::multiple(5,6);
    println!("babo_lib_3::multiple : {value}");
}

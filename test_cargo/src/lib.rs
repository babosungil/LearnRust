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


use std::thread;

fn main() {
    let mut var_mut = 1;        // mutable

    let var = 2;                // immutable
    println!("{var}");

    let var = var + 1;          // shadowing
    println!("{var}");

    let str = "! test ?";
    println!("{str}");

    let str = str.len();        // shadowing
    println!("{str}");

    const TEST_CONSTANT_VALUE:i32 = 10 * 20 / 5;    // constant
    println!("{TEST_CONSTANT_VALUE}");
}
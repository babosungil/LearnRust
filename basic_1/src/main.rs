//use std::string;


fn main() {
    let mut var_mut = 1;        // mutable
    println!("{var_mut}");
    var_mut = 2;
    println!("{var_mut}");

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

    let integer32:i32 = 9876;
    println!("{integer32}");

    let mut integer8:i8 = 125;
    integer8 += 1;
    println!("{integer8}");
    //integer8 += 127;                    //overflow    

    let tuple_test: (i8, f32, char, String) = (12, 3.4, 'a', ("string_test").to_string());
    println!("{:?}", tuple_test);

}
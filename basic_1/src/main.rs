//use std::string;
//use std::array;

//use std::io::{self, Read};
//use std::io;

fn main() {
    let mut var_mut = 1;        // mutable
    println!("{var_mut}");
    var_mut = 2;
    println!("{var_mut}");

    let var = 3;                // immutable
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
    println!("{:?}, {:?}, {:?}, {:?}", tuple_test.0, tuple_test.1, tuple_test.2, tuple_test.3);

    let array_test = [1, 2, 3];
    println!("{:?}", array_test);
    println!("{:?}", array_test[2]);
    let array_test2: [u8; 4] = [6,7,8,10];
    println!("{:?}", array_test2);


    let mut input_index = String::new();

    std::io::stdin()
    .read_line(&mut input_index)
    .expect("failed to read line!");

    let index_test: usize = input_index
    .trim()
    .parse()
    .expect("not a number!");

    let test_error = array_test2[index_test];    // if > 4 makes panic!
    println!("entered number");
    println!("{test_error}");
}
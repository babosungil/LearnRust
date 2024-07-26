use core::panic;
use std::{fs::File, io::ErrorKind, ptr::{null, NonNull}};

/*
enum Result<T,E> {
    Ok(T),
    Error(E),
}
*/

fn main() {
    //panic!("panic!");     // occur panic

    let vec_temp = vec![1, 2, 3, 4, 5];
    //vec_temp[10];           // error - panic


    let file_open_result = File::open("new_file.txt");
    let file_open = match file_open_result {
        Ok(file) => {
            println!("file open success");
            file
        },
        Err(error) => match error.kind() { 
            ErrorKind::NotFound => match File::create("new_file.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("failed create new file : {:?}", e),
            },
            other_error => {
                panic!("error - other_error {:?}", other_error);
            }
        }
    };

    //let file_open_result = File::open("new_file_2.txt").expect("file open exception !!");
}

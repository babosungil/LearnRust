use core::panic;
use std::{fs::{self, File}, io::{self, ErrorKind, Read, Write}, ptr::{null, NonNull}};

fn read_propagating() -> Result<String, io::Error> {
    let result = File::open("propagating.txt");
    let mut open_file = match result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut read_text = String::new();
    match open_file.read_to_string(&mut read_text) {
        Ok(_) => Ok(read_text),
        Err(e) => Err(e),
    }
}


fn read_propagating_with_qm() -> Result<String, io::Error> {
    let mut result = File::open("propagating.txt")?;
    let mut read_text = String::new();
    result.read_to_string(&mut read_text)?;
    Ok(read_text)
}

fn read_propagating_simple() -> Result<String, io::Error> {
    fs::read_to_string("propagating.txt")
}



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


    let read_text = read_propagating();
    if read_text.is_ok() {
        println!("file data : {}", read_text.unwrap());
    }
    else {
        println!("{:?}", read_text.err());
    }

    let read_text_qm = read_propagating_with_qm();
    println!("file data qm {}", read_text_qm.unwrap());

    let read_test_simple = read_propagating_simple();
    println!("file data simple {}", read_test_simple.unwrap());

}


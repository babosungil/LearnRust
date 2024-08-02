use std::{env, fs};

fn grap(find_text : &String, file_name : &String) {
    let file_text = fs::read_to_string(file_name)
        .expect("file not exits");

    println!("{}", file_text);
}

fn main() {
    let args : Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() < 4 {
        println!("invalid parameter");
        return;
    }

    let command = &args[1];
    let find_text = &args[2];
    let file_name = &args[3];

    match command.as_str() {
        "grap" => {
            grap(find_text, file_name);
        } ,
        other => {},
    }
}

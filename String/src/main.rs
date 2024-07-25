fn main() {
    let mut str_1 = String::from("rust");
    str_1.push_str(" string");
    println!("str_1 : {str_1}");

    let str_2 = str_1 + &String::from(" testing...");
    println!("str_2 : {str_2}");
    //println!("str_1 : {str_1}");  // error - already move value
    //let str_3 = str_2[2];         // error - invalid syntex

    //let str_3 = String::from("1. बारिश का दिन है।");
    let str_3 = "1. बारिश का दिन है।";
    //println!("str_3[0] {:?}", str_3[0]);    // error - string cannot access direct index
    //println!("str_3[0] {:?}", str_3[0..3]); // error - unknow str_3 size
    for i in str_3.chars() {
        println!("str_3 for : {i}");
    }
}
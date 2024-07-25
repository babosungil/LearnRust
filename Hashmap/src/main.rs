use std::collections::HashMap;

fn main() {

    let mut hash_data = HashMap::new();
    hash_data.insert(String::from("one"), 1);
    hash_data.insert(String::from("three"), 3);

    let five_str = String::from("five");
    hash_data.insert(five_str, 5);
    //println!("five_str : {five_str}");  // error - value moved!

    let three = hash_data.get(&String::from("three"));
    println!("find three : {}", three.copied().unwrap_or(0));

    for (k, v) in &hash_data {
        println!("{k} : {v}");
    }


    hash_data.insert(String::from("three"), 33);
    println!("insert - {:?}", hash_data);


    hash_data.entry(String::from("three")).or_insert(333);
    hash_data.entry(String::from("seven")).or_insert(7);
    println!("entry - {:?}", hash_data);


    let seven = hash_data.entry(String::from("seven")).or_insert(7);
    *seven += 1;
    println!("entry - {:?}", hash_data);
}

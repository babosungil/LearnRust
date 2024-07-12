
// struct
struct UserData {
    active : bool,
    name : String,
    age : u8,
    point: i64,
}


// tuple struct
struct THREE_1 (i32, i32, i32);
struct THREE_2 (i32, i32, i32);


fn main() {

    // struct
    let active = true;
    let mut user_data_1 = UserData {
        active,
        name : String::from("my_name_1"),
        age : 32,
        point : 99999
    };
    let user_data_2 = UserData {
        active : false,
        name : String::from("my_name_2"),
        ..user_data_1
    };
    println!("user data - name {0}", user_data_1.name);
    user_data_1 = user_data_2;
    println!("user data - name {0}", user_data_1.name);
    //println!("user data - name {0}", user_data_2.name); // error - value borrowed


    // tuple struct
    let three_1 = THREE_1(10, 20, 30);
    let three_2 = THREE_2(300, 200, 100);
    println!("{}", three_1.0);
    //three_2 = three_1; // error!

}

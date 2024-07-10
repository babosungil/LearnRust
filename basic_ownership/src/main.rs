fn main() {

    // scope example
    {
        let var_1 = "sample1";
        println!("in scope var_1 : {var_1}");
    }
    // var_1 is already dropped > next code is error
    //println!("out scope var_1 : {var_1}");

    
    // move
    let /*mut*/ move_1: String = String::from("move test");
    let mut move_2: String = move_1; // now move_1 is unavailed
    //move_1.push_str(", move_1");
    move_2.push_str(", move_2");
    println!("move : {move_2}");


    // clone
    let mut clone_1 = String::from("copy test");
    let mut clone_2 = clone_1.clone();
    clone_1 += ", clone_1";
    clone_2 += ", clone_2";
    println!("clone : {clone_1} || {clone_2}");


    // copy
    let copy_1 = 1;
    let copy_2 = copy_1;
    println!("copy : {}, {}", copy_1, copy_2);

    let copy_3 = (11, 22, 33);
    let copy_4 = copy_3;
    println!("copy : {:?}, {:?}", copy_3, copy_4);

}

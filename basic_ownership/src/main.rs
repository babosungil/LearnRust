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


    // with function
    let func_1 = 321;
    let func_2 = String::from("with function");
    func(func_1, func_2.clone());
    println!("func_2 : {func_2}");
    let func_3 = func(func_1, func_2);
    //println!("func_2 : {func_2}");        // error
    println!("func_3 : {func_3}");


    // reference
    let ref_1 = String::from("reference");
    let mut ref_2 = String::from("reference2");    
    func_ref(&ref_1, &mut ref_2);
    
    //func_ref(&ref_2, &mut ref_2); // error
    println!("ref_2 : {ref_2}");

    /* error - mutable borrow - able only one
    let ref_3 = &mut ref_2;
    let ref_4 = &mut ref_2;
    println!("{ref_3}, {ref_4}");
    */
}

fn func(param_1 : i32, mut param_2 : String) -> String {
    println!("call func - param_1 : {param_1}, param_2 : {param_2}");
    param_2.push_str(", push_str");
    param_2
}

fn func_ref(param_1 : &String, param_2 : &mut String) {
    println!("call func_ref - param_1 : {param_1}, param_2 : {param_2}");
    param_2.push_str(", push_str");
}
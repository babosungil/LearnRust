
fn main() {

    test_func(123, "it's string".to_string());


    let var_1 =  {
         let in_var_1 = 2;
         in_var_1 + 3               // check semicolon
    };
    println!("var_1 : {var_1}");


    let var_2 = return_func(var_1);
    println!("var_2 : {var_2}");


    let var_3 = if_func(12, 23);
    println!("var_3 : {var_3}");

    iter_func(6);
}

fn test_func(param_1 : i32, param_2 : String) {
    println!("{param_1} {param_2}");
}

fn return_func(param_1 : i32) -> i32 {
    param_1 + 123                    // check semicolon
}

fn if_func(param_1 : i32, param_2 : i32) -> i32 {
    if param_1 > param_2 {
        param_1
    }
    else if param_1 < param_2 {
        param_2
    }
    else {
        0
    }
}

fn iter_func(param_1 : i32) {

    let mut iter_counter = 0;

    let loop_result = loop {        
        iter_counter += 1;
        println!("loop {iter_counter}");        
        if iter_counter >= param_1 {
            break iter_counter
        }
    };
    println!("loop fin - count {loop_result}");

    iter_counter = 0;

    while iter_counter < param_1 {
        iter_counter += 1;
        println!("while {iter_counter}");
    }

    let datas = (1..=param_1).collect::<Vec<i32>>();
    //for element in (1..param_1) {
    for element in datas {
        println!("for {:?}", element);
    }

}

/*
invalid if condition value

fn if_2_func() -> String {
    if 5 {
        "true".to_string()
    }
    else {
        "false".to_string()
    }
}
*/
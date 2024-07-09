
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
    else {
        param_2
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
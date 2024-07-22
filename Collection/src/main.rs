fn main() {

    let mut vec : Vec<i32> = Vec::new();
    vec.push(12);
    vec.push(23);
    vec.push(34);
    vec.push(45);
    vec.push(56);
    println!("vec {:?}", vec);

    //let vec_index = vec[9]; // panic!
    let vec_index = vec[2];
    println!("vec_index : {vec_index}");

    let vec_get = vec.get(7);
    match vec_get {
        Some(vec_get) => {
            println!("vec option match success!, {}", vec_get);
            let vec_get_2 = vec_get + 1;
            println!("vec_get_2 : {:?}", vec_get_2);
        },
        None => println!("vec option match fail!"),
    }

    let vec_macro = vec![98.7,87.6,76.5,65.4];
    println!("vec_macro {:?}", vec_macro);

    
    for i in vec.iter() {   // into_iter() // error! - move borrow
        println!("vec iter - {i}");
    }
    
    let vec_ref = &vec[1];
    println!("vec_ref {vec_ref}");

    vec.push(999);
    println!("{:?}", vec);

    //println!("vec_ref {vec_ref}"); // error - mutable borrow occur


    #[derive(Debug)]
    enum ETest {
        Integer(i32),
        Float(f64),
        Text(String),
    }

    let test_data = vec![
        ETest::Integer(123),
        ETest::Float(1.234),
        ETest::Text(String::from("12345")),
    ];    
    println!("test_data : {:?}, {:?}, {:?}", test_data[0], test_data[1], test_data[2]);
}

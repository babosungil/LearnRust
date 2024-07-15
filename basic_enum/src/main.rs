
#[derive(Debug)]
enum EEnumTest {
    EnumBase,
    TupleTest(i32, String),    
    StringTest(String),
    StructTest { a : i32, b : String },
}

#[derive(Debug)]
enum EOption<T> {
    None,
    Some(T)

}

fn main() {
    let enumTest = EEnumTest::EnumBase;    
    println!("enumTest {:?}", enumTest);
    let tupleTest = EEnumTest::TupleTest(1, String::from("hello?"));
    println!("tupleTest {:?}", tupleTest);
    let stringTest = EEnumTest::StringTest(String::from("rust!"));
    println!("stringTest {:?}", stringTest);
    let structTest = EEnumTest::StructTest { a: 222, b: String::from("struct test") };
    println!("structTest {:?}", structTest);


    let optionSome_i32 = EOption::Some(123);
    println!("optionSome_i32 : {:?}", optionSome_i32);
    let optionSome_String = EOption::Some(String::from("Optine - Some"));
    println!("optionSome_String : {:?}", optionSome_String);
    let optionSome_Char = EOption::Some('A');
    println!("optionSome_Char : {:?}", optionSome_Char);

    //let temp = 1 + optionSome_i32; // error - data type miss match
}

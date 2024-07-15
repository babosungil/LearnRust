
#[derive(Debug)]
enum EEnumTest {
    EnumBase,
    TupleTest(i32, String),    
    StringTest(String),
    StructTest { a : i32, b : String },
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
}

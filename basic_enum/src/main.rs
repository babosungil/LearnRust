
#[derive(Debug)]
enum EEnumTest {
    EnumBase,
    TupleTest(i32, String),    
    StringTest(String),
    StructTest { a : i32, b : String },
}

#[derive(Debug)]
enum NewOption<T> {
    None,
    Some(T)
}

fn match_option(a : Option<i32>) -> Option<i32> {
    match a {
        None => None,
        Some(i) => Some(i + 1),
    }
}

#[derive(PartialEq)]
enum ETeam {
    Team1 = 1,
    Team2 = 2,
    Team3 = 3,
}

enum ESport {
    Soccer,
    Baseball(ETeam),
    Basketball,
    Volleyball,
}

fn match_sport(sport : ESport) -> u8 {
    match sport {
        ESport::Soccer => 1,        
        ESport::Baseball(team) => {                    
            if team == ETeam::Team1 {
                2
            }
            else if team == ETeam::Team2 {
                3
            }            
            else if team == ETeam::Team3 {
                4
            }
            else {
                5
            }
        },
        ESport::Basketball => 8,
        ESport::Volleyball => {
            println!("Volleyball !!!");
            16
        },
    }
}


fn main() {
    let enum_test = EEnumTest::EnumBase;    
    println!("enum_test {:?}", enum_test);
    let tuple_test = EEnumTest::TupleTest(1, String::from("hello?"));
    println!("tuple_test {:?}", tuple_test);
    let string_test = EEnumTest::StringTest(String::from("rust!"));
    println!("string_test {:?}", string_test);
    let struct_test = EEnumTest::StructTest { a: 222, b: String::from("struct test") };
    println!("struct_test {:?}", struct_test);


    let option_i32 = NewOption::Some(123);
    println!("option_i32 : {:?}", option_i32);
    let option_string = NewOption::Some(String::from("Optine - Some"));
    println!("option_string : {:?}", option_string);
    let option_char = NewOption::Some('A');
    println!("option_char : {:?}", option_char);

    let option_1 = Some(123);
    let option_2 = match_option(option_1);
    println!("option_1 : {:?}, option_2 : {:?}", option_1, option_2);

    //let temp = 1 + optionSome_i32; // error - data type miss match


    let soccer_value = match_sport(ESport::Soccer);
    let basketball_value = match_sport(ESport::Basketball);
    println!("Soccer : {soccer_value}, Basketball : {basketball_value}");

    let volleyball_value = match_sport(ESport::Volleyball);
    println!("Volleyball : {volleyball_value}");

    let baseball_value = match_sport(ESport::Baseball(ETeam::Team2));
    println!("Baseball = Team2 : {baseball_value}");


    // if let
    if let soccer_value = 2 {
        println!("soccer = 2");
    }
    else {
        println!("soccer != 2");
    }

}

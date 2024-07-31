
/*
fn add<T1>(num_1 : T1, num_2 : T1) -> T1 {
    num_1 + num_2          // error - GenericDataType can not + opeator
}
*/

use std::fmt::Display;

struct DataType_One<T> {
    t1_1 : T,
    t1_2 : T,
}

impl DataType_One<i32> {
    fn add(&self) -> i32 {
        self.t1_1 + self.t1_2
    }
}


struct DataType_Two<T1, T2> {
    t1 : T1,
    t2 : T2,
}

impl DataType_Two<i32, f32>  {
    fn add(&self) -> f32 {
        self.t1 as f32 + self.t2
    }
}



pub trait ToText {    
    fn toText(&self) -> String;

    fn toText_2(&self) -> String {
        String::from("ToText::toText_2")
    }
}

pub trait ToDisplay {
    fn toDisplay(&self) -> String;
}

enum EDays {
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
    Sun,
}

pub struct Days {
    day : EDays,
}

impl ToText for Days {
    fn toText(&self) -> String {
        match self.day {
            EDays::Mon => String::from("Monday"),
            EDays::Tue => String::from("Tuesday"),
            EDays::Wed => String::from("Wednesday"),
            EDays::Thu => String::from("Thuesday"),
            EDays::Fri => String::from("Friday"),
            EDays::Sat => String::from("Saturday"),
            EDays::Sun => String::from("Sunday"),
        }
    }

    fn toText_2(&self) -> String {
        String::from("Days::toText_2")
    }
}

impl ToDisplay for Days {
    fn toDisplay(&self) -> String {
        String::from("Days::toDisplay")
    }
}


enum ENumber {
    One,
    Two,
    Three,
    Four,
    Five,
}

pub struct Number {
    num : ENumber,
}

impl ToText for Number {
    fn toText(&self) -> String {
        match self.num {
            ENumber::One => String::from("One"),
            ENumber::Two => String::from("Two"),
            ENumber::Three => String::from("Three"),
            ENumber::Four => String::from("Four"),
            ENumber::Five => String::from("Five"),
        }
    }
}



fn doText(toText : &impl ToText) {
    println!("doText : {}", toText.toText());
}

fn doText2(toText : &(impl ToText + ToDisplay)) {
    println!("doText2 : {}", toText.toText());
}

fn doTextGeneral<T : ToText>(param : &T) {
    println!("doTextGeneral : {}", param.toText());
}

fn doTextGeneral2<T : ToText + ToDisplay>(param : &T) {
    println!("doTextGeneral2 : {}", param.toText());
}

fn doTextGeneral3<T>(param : &T)
where
    T: ToText + ToDisplay
{
    println!("doTextGeneral3 : {}", param.toText());
}




fn main() {
    //let data = Datas { num_1 : 2, num_2 : 3.1 }; // error!
    let data_one = DataType_One { t1_1 : 2, t1_2 : 3 };
    let data_two = DataType_Two { t1 : 2, t2 : 3.4 };

    println!("data_one {}", data_one.add());
    println!("data_two {}", data_two.add());


    let days : Days = Days { day : EDays::Tue };    
    println!("days : {}", days.toText());
    println!("days 2 : {}", days.toText_2());

    let number : Number = Number { num : ENumber::Four };
    println!("number : {}", number.toText());
    println!("number 2 : {}", number.toText_2());


    doText(&days);
    doText(&number);
    doTextGeneral(&days);
    doTextGeneral(&number);
    doText2(&days);
    doTextGeneral2(&days);
    doTextGeneral3(&days);
}



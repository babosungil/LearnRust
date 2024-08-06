use std::thread;


#[derive(Debug, PartialEq, Copy, Clone)]
enum EShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts : Vec<EShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<EShirtColor>) -> EShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> EShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                EShirtColor::Red => num_red += 1,
                EShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            EShirtColor::Red
        }
        else {
            EShirtColor::Blue
        }
    }
}


fn main() {

    let mut inven = Inventory {
        shirts: vec![EShirtColor::Red, EShirtColor::Blue, EShirtColor::Blue],
    };

    println!("{:?}", inven.shirts);

    let user_pref_1 = Some(EShirtColor::Red);
    let give_away_1: EShirtColor = inven.giveaway(user_pref_1);
    println!("user 1 - {:?}", give_away_1);

    let user_pref_2 = Some(EShirtColor::Blue);
    let give_away_2: EShirtColor = inven.giveaway(user_pref_2);
    println!("user 2 - {:?}", give_away_2);

    let user_pref_3 = None;
    let give_away_3: EShirtColor = inven.giveaway(user_pref_3);
    println!("user 3 - {:?}", give_away_3);



    let closure_test_1 = |param: u32| -> u32 { param + 1 };
    let closure_test_2 = |param: u32| { param + 2 };
    let closure_test_3 = |param| { param + 3 };
    let closure_test_4 = |param| param;

    let test_1 = closure_test_1(123);
    println!("closure_test_1 : {test_1}");

    let test_2 = closure_test_2(123);
    println!("closure_test_2 : {test_2}");

    let test_3 = closure_test_3(123);
    println!("closure_test_3 : {test_3}");

    let test_4 = closure_test_4(123);
    println!("closure_test_4 : {test_4}");
    //let test_4_2 = closure_test_4(1.23);          // error! can not use
    //println!("closure_test_4_2 : {test_4_2}");

    //thread::spawn(move|| println!("in thread - {:?}", inven.shirts)).join().unwrap();     // error - borrow data
    
    println!("before {:?}", inven.shirts);
    let mut add_data = || inven.shirts.push(EShirtColor::Red);
    add_data();
    println!("after {:?}", inven.shirts);
}

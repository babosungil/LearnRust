
/*
fn add<T1>(num_1 : T1, num_2 : T1) -> T1 {
    num_1 + num_2          // error - GenericDataType can not + opeator
}
*/

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



fn main() {
    //let data = Datas { num_1 : 2, num_2 : 3.1 }; // error!
    let data_one = DataType_One { t1_1 : 2, t1_2 : 3 };
    let data_two = DataType_Two { t1 : 2, t2 : 3.4 };

    println!("data_one {}", data_one.add());
    println!("data_two {}", data_two.add());
}



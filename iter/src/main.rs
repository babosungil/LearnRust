#[derive(PartialEq, Debug)]
struct Human
{
    age : u8,
    name : String,
}

fn find_by_age(age : u8, datas : Vec<Human>) -> Vec<Human> {
    datas.into_iter().filter(|s| s.age == age).collect()
}


fn main() {
    let datas = vec![1, 10, 100, 1000];


    let datas_iter = datas.iter();
    for data in datas_iter {
        println!("data : {data}");
    }
    

    let mut datas_iter = datas.iter();
    println!("{:?}", datas_iter.next());
    println!("{:?}", datas_iter.next());
    println!("{:?}", datas_iter.next());
    println!("{:?}", datas_iter.next());
    println!("{:?}", datas_iter.next());

    
    let datas_iter = datas.iter();
    let datas_sum: i32 = datas_iter.sum();
    println!("datas_sum : {datas_sum}");


    let datas_iter = datas.iter();
    let datas_iter_2 : Vec<_> = datas_iter.map(|x| x + 1).collect();
    for data in datas_iter_2 {
        println!("data : {data}");
    }



    let humans = vec![
        Human {
            age : 22,
            name : String::from("John"),
        },
        Human {
            age : 27,
            name : String::from("Paul"),
        },
        Human {
            age : 22,
            name : String::from("Hai"),
        },
    ];

    let twentytwo = find_by_age(22, humans);

    println!("twentytwo : {:?}", twentytwo);
}

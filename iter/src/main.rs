
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
}

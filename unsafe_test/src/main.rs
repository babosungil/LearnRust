use core::slice;

static mut MY_NUMBER : i32 = 100;

fn add_to_my_number() {
    unsafe {
        MY_NUMBER += 1;
    }
}


fn split_at(values : &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    
    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

fn main() {
    unsafe {
        println!("MY_NUMBER : {MY_NUMBER}");
    }
        add_to_my_number();
    unsafe {
        println!("MY_NUMBER : {MY_NUMBER}");
    }    
    
    let mut datas = [1,2,3,4,5,6,7,8,9,10,11];
    let (left, right) = split_at(&mut datas, 7);
    println!("left : {:?}", left);
    println!("right : {:?}", right);
}

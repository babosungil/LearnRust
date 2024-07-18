mod tmath;
//use tmath::util::mul::mul; // error - call private function
use tmath::util::div::div;

fn main() {
    let tmath_func = tmath::tmath_func();
    println!("tmath_func : {tmath_func}");

    let plus = tmath::util::plus(5, 2);
    println!("plus : {plus}");

    let minus = tmath::util::minus::minus(5, 2);
    println!("minus : {minus}");

    //let mul = mul(5, 2);          // error - call private function
    //println!("mul : {mul}");

    let div = div(5, 2);
    println!("div : {div}");
}

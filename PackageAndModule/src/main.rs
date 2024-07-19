mod tmath;
use rand::Rng;
//use tmath::util::mul::mul; // error - private function
//use tmath::util::* // glob operator
use tmath::util::div::div as util_div;
pub use tmath::util::minus;


fn main() {
    let math_data = tmath::util::MathData {
        num_one : 678,
        num_two : 890
    };
    println!("math_data : {0}, {1}", math_data.num_one, math_data.num_two);

    let tmath_func = tmath::tmath::tmath_func();
    println!("tmath_func : {tmath_func}");

    let plus = tmath::util::plus(5, 2);
    println!("plus : {plus}");

    let minus = tmath::util::minus::minus(5, 2);
    println!("minus : {minus}");

    //let mul = mul(5, 2);          // error - call private function
    //println!("mul : {mul}");

    let div = util_div(5, 2);
    println!("div : {div}");

    //let private_plus = tmath::private_util::plus(5, 3);     // error - call private module
    //println!("private_plus : {private_plus}");

    let super_tmath_func = tmath::util::super_tmath_func();
    println!("super_tmath_func : {super_tmath_func}");

    let rand_value = rand::thread_rng().gen_range(1..=1000);
    println!("rand_value {rand_value}");
}

pub mod util {

    struct MathData {
        num_one : i32,
        num_two : i32
    }

    pub fn plus(param_1 : i32, param_2 : i32) -> i32 {
        param_1 + param_2
    }

    pub mod minus {
        pub fn minus(param_1 : i32, param_2 : i32) -> i32 {
            param_1 - param_2
        }
    }

    pub mod mul {
        fn mul(param_1 : i32, param_2 : i32) -> i32 {
            param_1 * param_2
        }
    }

    pub mod div {
        pub fn div(param_1 : i32, param_2 : i32) -> i32 {
            param_1 / param_2
        }
    }    
}


pub fn tmath_func() -> i32 {
    987
}

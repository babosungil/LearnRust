pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn doPanic(signal : i32) -> i32 {
    match signal {
        1 => {
            panic!("doPanic - signal 1");
            1
        }
        other => {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_test_eq() {
        let result = add(2, 3);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_test_ne() {
        let result = add(3, 2);
        assert_ne!(result, 4);
    }

    #[test]
    fn test_panic() {
        let result = doPanic(1);
    }
}

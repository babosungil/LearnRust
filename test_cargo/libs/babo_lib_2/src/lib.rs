pub fn minus(left: usize, right: usize) -> usize {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = minus(100, 9);
        assert_eq!(result, 4);
    }
}

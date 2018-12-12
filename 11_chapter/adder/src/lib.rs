pub fn add_us(i: i32, j: i32) -> i32 {
    internal_add_us(i, j)
}

fn internal_add_us(i: i32, j: i32) -> i32 {
    i + j
}

pub fn panicking_fn() {
    panic!("This function panics for test.");
}

#[cfg(test)]
mod tests {
    use crate::add_us;
    use crate::panicking_fn;

    //#[test] attribute indicates this is a test function,
    //so the test runner knows to treat this function as a test.

    #[test]
    fn add_us_when_passed_2_and_2_should_return_4() {
        assert_eq!(add_us(2, 2), 4);
    }

    #[test]
    fn add_us_when_passed_2_and_2_should_not_return_5() {
        assert_ne!(add_us(2, 2), 5);
    }

    #[test]
    fn another_test() {
        assert!(
            2 >= 3,
            "Should fail, as 2 is never equal or greater than 3."
        );
    }

    #[test]
    #[ignore]
    fn another_test_too() {
        assert!(
            2 >= 3,
            "Should fail, as 2 is never equal or greater than 3."
        );
    }

    #[test]
    #[should_panic]
    fn test_should_not_fail_as_panic_is_expected() {
        panicking_fn();
    }

    //tesing private functions
    use super::internal_add_us;
    #[test]
    fn private_fn_test() {
        assert_eq!(internal_add_us(2, 40), 42);
    }
}

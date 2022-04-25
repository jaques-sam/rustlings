// iterators3.rs
// This is a bigger exercise than most of the others! You can do it!
// Here is your mission, should you choose to accept it:
// 1. Complete the divide function to get the first four tests to pass.
// 2. Get the remaining tests to pass by completing the result_with_list and
//    list_of_results functions.
// Execute `rustlings hint iterators3` to get some hints!

#[derive(Debug, PartialEq, Eq)]
pub enum DivisionError {
    NotDivisible { dividend: i32, divisor: i32 },
    DivideByZero,
}

// Calculate `a` divided by `b` if `a` is evenly divisible by `b`.
// Otherwise, return a suitable error.
pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        return Err(DivisionError::DivideByZero);
    }

    match a % b {
        x if x > 0 => Err(DivisionError::NotDivisible {
            dividend: a,
            divisor: b,
        }),
        x => Ok(a / b),
    }
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output is e.g.: Ok([1, 11, 1426, 3])
fn result_with_list(numbers: &Vec<i32>) -> Result<Vec<i32>, DivisionError> {
    numbers.into_iter().map(|&n| divide(n, 27)).collect()
}

// Complete the function and return a value of the correct type so the test passes.
// Desired output is e.g.: [Ok(1), Ok(11), Ok(1426), Ok(3)]
fn list_of_results(numbers: &Vec<i32>) -> Vec<Result<i32, DivisionError>> {
    numbers.into_iter().map(|&n| divide(n, 27)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(divide(81, 9), Ok(9));
    }

    #[test]
    fn test_not_divisible() {
        assert_eq!(
            divide(81, 6),
            Err(DivisionError::NotDivisible {
                dividend: 81,
                divisor: 6
            })
        );
    }

    #[test]
    fn test_divide_by_0() {
        assert_eq!(divide(81, 0), Err(DivisionError::DivideByZero));
    }

    #[test]
    fn test_divide_0_by_something() {
        assert_eq!(divide(0, 81), Ok(0));
    }

    #[test]
    fn test_result_with_list() {
        let numbers = vec![27, 297, 38502, 81];
        assert_eq!(
            format!("{:?}", result_with_list(&numbers)),
            "Ok([1, 11, 1426, 3])"
        );
    }

    #[test]
    fn test_list_of_results() {
        let numbers = vec![27, 297, 38502, 81];
        assert_eq!(
            format!("{:?}", list_of_results(&numbers)),
            "[Ok(1), Ok(11), Ok(1426), Ok(3)]"
        );
    }

    // extra tests
    #[test]
    fn test_result_with_list_with_error() {
        let numbers = vec![27, 297, 666, 81];
        assert_eq!(
            format!("{:?}", list_of_results(&numbers)),
            "[Ok(1), Ok(11), Err(NotDivisible { dividend: 666, divisor: 27 }), Ok(3)]"
        );
    }

    #[test]
    fn test_list_of_results_with_error() {
        let numbers = vec![9, 297, 38502, 81];
        assert_eq!(
            format!("{:?}", list_of_results(&numbers)),
            "[Err(NotDivisible { dividend: 9, divisor: 27 }), Ok(11), Ok(1426), Ok(3)]"
        );
    }
}

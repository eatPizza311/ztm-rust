// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    } else {
        Some(a / b)
    }
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{}{}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_clamp_lower() {
        let result = clamp(10, 15, 20);
        let expected = 15;
        assert_eq!(
            result, expected,
            "Value less than lower bound should be equal to lower bound."
        );
    }

    #[test]
    fn test_clamp_upper() {
        let result = clamp(25, 15, 20);
        let expected = 20;
        assert_eq!(
            result, expected,
            "Value larger than upper bound should be equal to upper bound."
        );
    }

    #[test]
    fn test_clamp_inside() {
        let result = clamp(15, 15, 20);
        let expected = 15;
        assert_eq!(result, expected, "Value inside the range should be itself.");
    }

    #[test]
    fn test_10_div_2() {
        let result = div(10, 2);
        let expected = Some(5);
        assert_eq!(result, expected, "10/2 should be 5.")
    }

    #[test]
    fn test_1_div_0() {
        let result = div(1, 0);
        let expected = None;
        assert_eq!(result, expected, "should catch divide by 0.")
    }

    #[test]
    fn test_concat() {
        let result = concat("me", "you");
        let expected = "meyou".to_owned();
        assert_eq!(result, expected, "should be meyou, got {result} instead.")
    }
}

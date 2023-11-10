use crate::math_operations::is_palindrome;

/// Problem 004: Largest Palindrome Product
///
/// ```
/// A palindromic number reads the same both ways.
/// The largest palindrome made from the product of two 2-digit
/// numbers is 9009 = 91 x 99.
/// ```
///
/// ## Example:
///
/// The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 x 99.
///
/// ## Note:
///
/// The largest palindrome made from the product of two 3-digit numbers.
pub fn solve() {
    println!("\n\t\t\tProblem 004: Largest Palindrome Product\n");

    println!(
        "Largest palindrome made from the product of two 3-digit numbers is {}",
        largest_palindrome(3)
    );
}

/// Returns the largest palindrome made from the product of two 3-digit numbers
fn largest_palindrome(digit: u32) -> i32 {
    let mut max = 1;
    let limit = 10_i32.pow(digit);

    for i in 1..limit {
        for j in 1..limit {
            let product = i * j;
            let palindrome = is_palindrome(product);
            if palindrome {
                if product > max {
                    max = product;
                }
            }
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::largest_palindrome;

    #[test]
    fn test_largest_palindrome() {
        assert_eq!(largest_palindrome(1), 9);
        assert_eq!(largest_palindrome(2), 9009);
    }
}
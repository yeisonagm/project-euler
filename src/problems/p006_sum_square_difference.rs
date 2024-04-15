/// Problem 006: Sum square difference
///
/// ```
/// The sum of the squares of the first ten natural numbers is,
/// 1^2 + 2^2 + ... + 10^2 = 385
/// The square of the sum of the first ten natural numbers is,
/// (1 + 2 + ... + 10)^2 = 55^2 = 3025
/// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the suma is
/// 3025 - 385 = 2640
/// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
/// ```
///
/// ## Example
/// The difference between the sum of the squares of the first ten natural numbers and the square of the sum is 2640.
///
/// ## Note
/// The difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
pub fn solve() {
    println!("\n\t\t\tProblem 006: Sum square difference\n");
    println!("The difference between the sum of the squares of the first one hundred natural numbers and the square of the sum is {}",
             sum_square_difference(100));
}

/// Returns the difference between the sum of the squares of the first n natural numbers and the square of the sum.
///
/// # Arguments
///
/// * `n` - A u32 integer that specifies the range of numbers from 1 to n.
///
/// # Example
///
/// ```
/// let result = sum_square_difference(3);
/// assert_eq!(result, 22);
/// ```
fn sum_square_difference(n: u32) -> u32 {
    // Sum of the squares of the first n natural numbers
    let sum_of_squares = (1..=n).map(|x| x.pow(2)).sum::<u32>();

    // Square of the sum of the first n natural numbers
    let square_of_sum = (1..=n).sum::<u32>().pow(2);

    square_of_sum - sum_of_squares
}

#[cfg(test)]
mod tests {
    use super::sum_square_difference;

    #[test]
    fn test_sum_square_difference() {
        assert_eq!(sum_square_difference(0), 0);
        assert_eq!(sum_square_difference(1), 0);
        assert_eq!(sum_square_difference(2), 4);
        assert_eq!(sum_square_difference(3), 22);
        assert_eq!(sum_square_difference(10), 2640);
    }

    #[test]
    #[should_panic]
    fn test_sum_square_difference_panic() {
        sum_square_difference(u32::MAX);
    }
}
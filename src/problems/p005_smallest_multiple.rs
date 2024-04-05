/// Problem 005: Smallest multiple
///
/// ```
/// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
/// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
/// ```
///
/// ## Example:
/// The smallest number that can be divided by each of the numbers from 1 to 10 is 2520.
///
/// ## Note
///
/// The smallest number that can be divided by each of the numbers from 1 to 20.
pub fn solve() {
    println!("\n\t\t\tProblem 005: Smallest multiple\n");
    println!(
        "The smallest number that can be divided by each of the numbers from 1 to 20 is {}",
        smallest_multiple(20)
    );
}

/// Returns the smallest number that can be divided by each of the numbers from 1 to n.
///
/// # Arguments
///
/// * `n` - A u32 integer that specifies the range of numbers from 1 to n.
///
/// # Example
///
/// ```
/// let result = smallest_multiple(3);
/// assert_eq!(result, 6);
/// ```
fn smallest_multiple(n: u32) -> u32 {
    let mut numbers = (2..=n).collect::<Vec<u32>>();
    let mut smallest = 1;
    let mut divider = 2;

    // Loop until all numbers in the vector are 1.
    while numbers.iter().any(|&e| e != 1) {
        // If no number in the vector is divisible by the divider, increment the divider.
        if !numbers.iter().any(|&e| e % divider == 0) {
            divider += 1;
            continue;
        }

        // Divide all numbers in the vector that are divisible by the divider.
        for i in 0..numbers.len() {
            if (numbers[i] % divider) == 0 {
                numbers[i] /= divider;
            }
        }

        smallest *= divider;
    }

    smallest
}

#[cfg(test)]
mod tests {
    use super::smallest_multiple;

    #[test]
    fn test_smallest_multiple() {
        assert_eq!(smallest_multiple(1), 1);
        assert_eq!(smallest_multiple(2), 2);
        assert_eq!(smallest_multiple(3), 6);
        assert_eq!(smallest_multiple(4), 12);
        assert_eq!(smallest_multiple(5), 60);
        assert_eq!(smallest_multiple(6), 60);
        assert_eq!(smallest_multiple(7), 420);
        assert_eq!(smallest_multiple(8), 840);
        assert_eq!(smallest_multiple(9), 2520);
        assert_eq!(smallest_multiple(10), 2520);
        assert_eq!(smallest_multiple(11), 27720);
        assert_eq!(smallest_multiple(12), 27720);
        assert_eq!(smallest_multiple(13), 360360);
    }
}

/// Calculates the nth term of the Fibonacci sequence.
///
/// The Fibonacci sequence is a series of numbers where each term is the sum of the two
/// preceding terms. The sequence typically starts with 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, ...
///
/// # Parameters
///
/// - `n`: The position of the desired term in the Fibonacci sequence (1-based index).
///
/// # Returns
///
/// The value of the nth term in the Fibonacci sequence.
///
/// # Panics
///
/// If `n` is 0, this function will panic because there is no 'F0' in the Fibonacci sequence.
///
/// # Examples
///
/// ```
/// use project_euler::math_operations::fibonacci;
///
/// assert_eq!(fibonacci(1), 1);
/// assert_eq!(fibonacci(3), 2);
/// assert_eq!(fibonacci(10), 55);
/// ```
pub fn fibonacci(n: u32) -> u32 {
    // Fn = Fn-1 + Fn-2
    if n == 0 {
        panic!("Invalid fibonacci -> 'F0 not exist'");
    }
    if n == 1 || n == 2 {
        return 1;
    }
    return fibonacci(n - 1) + fibonacci(n - 2);
}

/// Calculates the Fibonacci sequence up to a given limit.
///
/// This function generates a vector of Fibonacci sequence values where each value
/// is less than the specified limit `n`.
///
/// # Parameters
///
/// - `n`: Representing the limit for Fibonacci values.
///
/// # Returns
///
/// A `Vec<u32>` containing the Fibonacci sequence values up to but not exceeding `n`.
///
/// # Example
///
/// ```
/// use project_euler::math_operations::fibonacci_terms;
///
/// let n = 10;
/// let fibonacci_sequence = fibonacci_terms(n);
/// assert_eq!(fibonacci_sequence, vec![1, 1, 2, 3, 5, 8]);
/// ```
///
/// # Note
///
/// The Fibonacci sequence is a series of numbers in which each number is the sum
/// of the two preceding ones, usually starting with 1 and 1.
pub fn fibonacci_terms(n: u32) -> Vec<u32> {
    // Returns a vector with Fibonacci sequence values less than `n`.
    // Each value is less than the specified limit `n`.
    (1..)
        .map(|e| fibonacci(e))
        .take_while(|&fi| fi < n)
        .collect()
}

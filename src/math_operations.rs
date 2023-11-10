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

/// Checks if a given integer is a prime number.
///
/// This function determines whether the input integer is a prime number by
/// checking if it has any divisors other than 1 and itself.
///
/// # Parameters
///
/// - `number`: A generic integer type that you want to check for primality.
///
/// # Returns
///
/// A boolean value `true` if the input number is prime, `false` otherwise.
///
/// # Example
///
/// ```
/// use project_euler::math_operations::is_prime;
///
/// let prime_number: u64 = 17;
/// let non_prime_number: u32 = 4;
///
/// assert_eq!(is_prime(prime_number), true);
/// assert_eq!(is_prime(non_prime_number), false);
/// ```
///
/// # Note
///
/// A prime number is a positive integer greater than 1 that has no positive
/// divisors other than 1 and itself. The function returns `false` for numbers
/// less than 2, as they are not prime by definition.
pub fn is_prime<T>(number: T) -> bool
where
    T: Into<u64>,
{
    let number: u64 = number.into();
    if number < 2 {
        return false;
    }
    let n = (number as f64).sqrt().abs() as u64;

    for i in 2..=n {
        if number % i == 0 {
            return false;
        }
    }
    true
}

/// Checks if a number is a palindrome.
///
/// A palindrome is a number that reads the same forward as backward.
///
/// # Arguments
///
/// * `number` - The number to check for palindrome property.
///
/// # Returns
///
/// Returns `true` if the number is a palindrome, and `false` otherwise.
///
/// # Examples
///
/// ```
/// use project_euler::math_operations::is_palindrome;
///
/// assert_eq!(is_palindrome(121), true);
/// assert_eq!(is_palindrome(123), false);
/// ```
///
/// # Note
///
/// This function treats negative numbers as palindromic, considering only the absolute value.
/// ```
/// use project_euler::math_operations::is_palindrome;
/// assert_eq!(is_palindrome(-121), true);
/// ```
pub fn is_palindrome(number: i32) -> bool {
    let num = number.abs().to_string();
    let reversed = num.chars().rev().collect::<String>();
    num == reversed
}

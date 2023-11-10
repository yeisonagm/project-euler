use project_euler::math_operations::*;

/// Tests for the `fibonacci` function.
#[test]
fn test_fibonacci() {
    assert_eq!(1, fibonacci(1));
    assert_eq!(1, fibonacci(2));
    assert_eq!(2, fibonacci(3));
    assert_eq!(3, fibonacci(4));
    assert_eq!(5, fibonacci(5));
    assert_eq!(89, fibonacci(11));
}

/// Tests for the `fibonacci` function that are expected to panic.
#[test]
#[should_panic]
fn test_fibonacci_panic() {
    fibonacci(0);
}

/// Tests for the `fibonacci_terms` function.
#[test]
fn test_fibonacci_terms() {
    assert_eq!(fibonacci_terms(0), vec![]);
    assert_eq!(fibonacci_terms(2), vec![1, 1]);
    assert_eq!(fibonacci_terms(5), vec![1, 1, 2, 3]);
    assert_eq!(fibonacci_terms(10), vec![1, 1, 2, 3, 5, 8]);
    let terms = vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
    assert_eq!(fibonacci_terms(100), terms);
}

/// Test cases for the is_prime function.
#[test]
fn test_is_prime() {
    // Prime numbers
    assert_eq!(is_prime(2u64), true);
    assert_eq!(is_prime(11u64), true);
    assert_eq!(is_prime(131u64), true);
    assert_eq!(is_prime(1009u64), true);
    assert_eq!(is_prime(65537u64), true);
    assert_eq!(is_prime(1000003u64), true);

    // Non-prime numbers
    assert_eq!(is_prime(1u64), false);
    assert_eq!(is_prime(18u64), false);
    assert_eq!(is_prime(121u64), false);
    assert_eq!(is_prime(299u64), false);
    assert_eq!(is_prime(9999u64), false);
    assert_eq!(is_prime(10017u64), false);
}

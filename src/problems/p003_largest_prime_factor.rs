use crate::math_operations::is_prime;

/// Problem 003: Large prime factors
///
/// ```
/// The prime factors of 13195 are 5, 7, 13 and 29.
/// What is the largest prime factor of the number 600851475143?
/// ```
///
/// ## Example:
/// Large prime factor of 13195 is 29.
///
/// ## Note:
/// Show the largest prime factor of the number 600851475143.
pub fn solve() {
    let number = 600851475143;
    let primes_factors = primes_factors(number);
    println!("The prime factors of {} are: {:?}", number, primes_factors);
    println!(
        "Largest prime factor the number {} is {}.",
        number,
        primes_factors.last().unwrap()
    );
}

/// Returns a list of prime numbers factors of 'number'.
fn primes_factors(number: u64) -> Vec<u64> {
    let mut primes_factors = Vec::new();
    let n: u64 = (number as f64).sqrt().abs() as u64;

    for num in 2..n {
        if number % num == 0 && is_prime(num) {
            primes_factors.push(num);
        }
    }

    primes_factors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primes_factors() {
        assert_eq!(Vec::<u64>::new(), primes_factors(0));
        assert_eq!(vec![2], primes_factors(10));
        assert_eq!(vec![2, 5], primes_factors(100));
        assert_eq!(vec![5, 7, 13, 29], primes_factors(13195));
    }
}

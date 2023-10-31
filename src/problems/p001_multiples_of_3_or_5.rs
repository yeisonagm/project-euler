/// # Problem 001: Multiples of 3 or 5
/// 
/// ```
/// Find the sum of all the multiples of 3 or 5 below 1000.
/// ```
///
/// ## Example:
/// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6, and 9.
/// The sum of these multiples is 23.
///
/// ## Note:
/// The sum of all the multiples of 3 or 5 below the specified limit.
pub fn solve() { 
    print!("The sum of multiples below than 10 is ");
    let multiples_10 = multiples_3or5(10);
    println!("{}", multiples_10.into_iter().sum::<u32>());
    
    print!("The sum of multiples below than 1000 is ");
    let multiples_1000 = multiples_3or5(1000);
    println!("{}", multiples_1000.into_iter().fold(0, |a, b| a + b));
}


fn multiples_3or5(last: u32) -> Vec<u32> {
(3..last).filter(|&e| e % 3 == 0 || e % 5 == 0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiples_3or5() {
        assert_eq!(multiples_3or5(10), vec![3, 5, 6, 9]);
        assert_eq!(multiples_3or5(20), vec![3, 5, 6, 9, 10, 12, 15, 18]);
        assert_eq!(multiples_3or5(0), vec![]);
        assert_eq!(multiples_3or5(2), vec![]);
    }

    #[test]
    fn test_sum_multiples_3or5() {
        assert_eq!(multiples_3or5(10).into_iter().sum::<u32>(), 23);
        assert_eq!(multiples_3or5(20).into_iter().sum::<u32>(), 78);
        assert_eq!(multiples_3or5(0).into_iter().sum::<u32>(), 0);
        assert_eq!(multiples_3or5(2).into_iter().sum::<u32>(), 0);
    }
}
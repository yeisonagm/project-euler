/// Returns the nth fibonacci term
/// 
/// Fibonacci sequence: 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, ...
/// 
/// # Examples
/// 
/// ```
/// fibonacci(1) = 1
/// fibonacci(3) = 2
/// fibonacci(10) = 55
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
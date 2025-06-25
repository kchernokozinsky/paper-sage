/// Calculate the factorial of a given number
/// 
/// # Arguments
/// * `n` - The number to calculate factorial for
/// 
/// # Returns
/// * `Option<u64>` - Some(factorial) if successful, None if overflow occurs
/// 
/// # Examples
/// ```
/// assert_eq!(factorial(5), Some(120));
/// assert_eq!(factorial(0), Some(1));
/// ```
pub fn factorial(n: u64) -> Option<u64> {
    if n == 0 || n == 1 {
        return Some(1);
    }
    
    let mut result: u64 = 1;
    for i in 2..=n {
        result = result.checked_mul(i)?;
    }
    
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_factorial() {
        assert_eq!(factorial(5), Some(120));
        assert_eq!(factorial(0), Some(1));
        assert_eq!(factorial(1), Some(1));
    }
    
    #[test]
    fn test_overflow() {
        // This should return None due to overflow
        assert_eq!(factorial(21), None);
    }
} 
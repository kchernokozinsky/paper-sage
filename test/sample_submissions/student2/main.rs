use std::io;

fn find_max(numbers: &[i32]) -> Option<i32> {
    if numbers.is_empty() {
        return None;
    }
    
    let mut max = numbers[0];
    for &num in numbers.iter().skip(1) {
        if num > max {
            max = num;
        }
    }
    
    Some(max)
}

fn main() {
    println!("Enter numbers separated by spaces:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    
    match find_max(&numbers) {
        Some(max) => println!("Maximum value: {}", max),
        None => println!("No numbers provided"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_max() {
        assert_eq!(find_max(&[1, 2, 3, 4, 5]), Some(5));
        assert_eq!(find_max(&[-1, -2, -3]), Some(-1));
        assert_eq!(find_max(&[42]), Some(42));
        assert_eq!(find_max(&[]), None);
    }
    
    #[test]
    fn test_find_max_with_duplicates() {
        assert_eq!(find_max(&[1, 1, 1]), Some(1));
        assert_eq!(find_max(&[5, 3, 5, 2]), Some(5));
    }
} 
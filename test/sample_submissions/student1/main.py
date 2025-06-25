def calculate_sum(numbers):
    """
    Calculate the sum of a list of numbers.
    
    Args:
        numbers (list): A list of numbers to sum
        
    Returns:
        float: The sum of all numbers in the list
        
    Raises:
        ValueError: If the input is not a list or contains non-numeric values
    """
    if not isinstance(numbers, list):
        raise ValueError("Input must be a list")
    
    if not numbers:
        return 0
    
    total = 0
    for num in numbers:
        if not isinstance(num, (int, float)):
            raise ValueError("All elements must be numbers")
        total += num
    
    return total

# Test cases
if __name__ == "__main__":
    # Test with normal input
    print(calculate_sum([1, 2, 3, 4, 5]))  # Should print 15
    
    # Test with empty list
    print(calculate_sum([]))  # Should print 0
    
    # Test with negative numbers
    print(calculate_sum([-1, -2, 3]))  # Should print 0
    
    # Test error handling
    try:
        print(calculate_sum("not a list"))  # Should raise ValueError
    except ValueError as e:
        print(f"Error: {e}")
    
    try:
        print(calculate_sum([1, "two", 3]))  # Should raise ValueError
    except ValueError as e:
        print(f"Error: {e}") 
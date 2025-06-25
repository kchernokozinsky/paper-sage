def factorial(n):
    """
    Calculate the factorial of a given number.
    
    Args:
        n (int): The number to calculate factorial for
        
    Returns:
        int: The factorial of n
        
    Raises:
        ValueError: If n is negative
    """
    if n < 0:
        raise ValueError("Factorial is not defined for negative numbers")
    
    if n == 0 or n == 1:
        return 1
    
    result = 1
    for i in range(2, n + 1):
        result *= i
    
    return result

# Test cases
if __name__ == "__main__":
    print(factorial(5))  # Should print 120
    print(factorial(0))  # Should print 1
    try:
        print(factorial(-1))  # Should raise ValueError
    except ValueError as e:
        print(f"Error: {e}") 
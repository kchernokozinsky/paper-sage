package com.example;

/**
 * Calculator class that provides basic arithmetic operations
 * Demonstrates good object-oriented design and error handling.
 */
public class Calculator {
    
    /**
     * Performs arithmetic operations on two numbers
     * @param a first operand
     * @param b second operand
     * @param operation the arithmetic operation to perform
     * @return result of the calculation
     * @throws IllegalArgumentException if operation is not supported
     * @throws ArithmeticException if division by zero is attempted
     */
    public double calculate(double a, double b, String operation) {
        switch (operation) {
            case "+":
                return add(a, b);
            case "-":
                return subtract(a, b);
            case "*":
                return multiply(a, b);
            case "/":
                return divide(a, b);
            default:
                throw new IllegalArgumentException("Unsupported operation: " + operation);
        }
    }
    
    /**
     * Adds two numbers
     * @param a first number
     * @param b second number
     * @return sum of the numbers
     */
    private double add(double a, double b) {
        return a + b;
    }
    
    /**
     * Subtracts second number from first
     * @param a first number
     * @param b second number
     * @return difference of the numbers
     */
    private double subtract(double a, double b) {
        return a - b;
    }
    
    /**
     * Multiplies two numbers
     * @param a first number
     * @param b second number
     * @return product of the numbers
     */
    private double multiply(double a, double b) {
        return a * b;
    }
    
    /**
     * Divides first number by second
     * @param a dividend
     * @param b divisor
     * @return quotient of the division
     * @throws ArithmeticException if divisor is zero
     */
    private double divide(double a, double b) {
        if (b == 0) {
            throw new ArithmeticException("Division by zero is not allowed");
        }
        return a / b;
    }
} 
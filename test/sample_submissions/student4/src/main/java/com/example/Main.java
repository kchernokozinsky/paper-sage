package com.example;

import java.util.Scanner;

/**
 * Main class for the student assignment
 * Demonstrates good programming practices including documentation,
 * error handling, and clean code structure.
 */
public class Main {
    
    /**
     * Main method that demonstrates various programming concepts
     * @param args command line arguments
     */
    public static void main(String[] args) {
        System.out.println("Welcome to the Student Assignment!");
        
        try {
            Calculator calculator = new Calculator();
            Scanner scanner = new Scanner(System.in);
            
            System.out.print("Enter first number: ");
            double num1 = scanner.nextDouble();
            
            System.out.print("Enter second number: ");
            double num2 = scanner.nextDouble();
            
            System.out.print("Enter operation (+, -, *, /): ");
            String operation = scanner.next();
            
            double result = calculator.calculate(num1, num2, operation);
            System.out.printf("Result: %.2f %s %.2f = %.2f%n", 
                            num1, operation, num2, result);
                            
        } catch (Exception e) {
            System.err.println("Error: " + e.getMessage());
        }
    }
} 
# Student Assignment - Calculator Application

## Overview
This project implements a simple calculator application in Java that demonstrates good programming practices including:
- Object-oriented design
- Error handling
- Documentation
- Clean code structure

## Project Structure
```
src/main/java/com/example/
├── Main.java          # Main application entry point
└── Calculator.java    # Calculator class with arithmetic operations
```

## Features
- Basic arithmetic operations (+, -, *, /)
- Input validation and error handling
- Division by zero protection
- User-friendly console interface

## How to Run
1. Compile the Java files:
   ```bash
   javac src/main/java/com/example/*.java
   ```

2. Run the application:
   ```bash
   java -cp src/main/java com.example.Main
   ```

## Usage
The program will prompt you to enter:
1. First number
2. Second number  
3. Operation to perform

It will then display the result of the calculation.

## Error Handling
The application handles various error cases:
- Invalid operations
- Division by zero
- Invalid input types 
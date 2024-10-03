fn factorial(n: i32) -> i32 {
    // Print the current value of n at the start of each function call
    println!("Called factorial with n = {}", n);

    // Base case: print the result when base case is reached
    if n <= 1 {
        println!("Reached base case with n = {}. Returning 1.", n);
        1
    } else {
        // Recursive case: calculate factorial(n - 1)
        let result = n * factorial(n - 1);
        println!("Returning {} * factorial({}) = {}", n, n - 1, result); // Show the result of each step
        result
    }
}

fn main() {
    let number = 5;
    let result = factorial(number);
    println!("Factorial of {} is: {}", number, result);
}
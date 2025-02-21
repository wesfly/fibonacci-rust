fn main() {
    println!("Fibonacci Sequence");
    let max_iterations = 185; // It won't go higher due to overflow
    for i in 1..=max_iterations {
        let x: u128 = fib(i); // Call the fib function with the current iteration number
        print!("Iteration {}: ", i);
        println!("{:#?}", x);
    }
}

fn fib(n: u128) -> u128 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib_rec(0, 1, n),
    } // Use a simple pattern matching to handle base cases and recursion
}

fn fib_rec(a: u128, b: u128, n: u128) -> u128 {
    if n == 0 {
        // Base case: If the remaining value is 0, return the accumulated sum
        return a;
    }
    let c = a + b; // Calculate the next Fibonacci number using previous two numbers
    fib_rec(b, c, n - 1) // Recursively calculate the next Fibonacci number
}

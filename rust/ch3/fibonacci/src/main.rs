
fn fibonacci(num: u32) -> u32 {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    }

    return fibonacci(num - 1) + fibonacci(num - 2);
}


fn main() {

    for num in 0..11 {
        println!("Fibonacci {}: {}", num, fibonacci(num));
    }

    // compile error println!("Fibonacci -1: {}", fibonacci(-1));
}

use std::time::Instant;

fn main() {
    let start = Instant::now();

    for num in 0..=44 {
        let fibonnaci = get_fibonacci(num);
        println!("Fibonacci for {num} is {fibonnaci}");
    }
    let duration = start.elapsed();
    println!("Execution time {:.2?}", duration);

    let start = Instant::now();
    for num in 0..=44 {
        let fibonnaci = get_fibonacci_recursive(num);
        println!("Fibonacci for {num} is {fibonnaci}");
    }

    let duration = start.elapsed();
    println!("Execution time {:.2?}", duration);
}

fn get_fibonacci_recursive(n: u32) -> u32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        get_fibonacci_recursive(n - 1) + get_fibonacci_recursive(n - 2)
    }
}

fn get_fibonacci(n: u32) -> u32 {
    let mut counter = n;
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut old_a: u32;

    let mut result: u32 = 0;

    if n < 2 {
        return n;
    }
    while counter > 0 {
        result = a;
        old_a = a;

        a = b;
        b = b + old_a;
        counter -= 1;
    }
    result
}

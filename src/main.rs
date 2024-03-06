fn main() {
    let mut base_of_fib = String::new();

    println!("Please input the base of fibonacci value");

    std::io::stdin()
        .read_line(&mut base_of_fib)
        .expect("Failed to read number of fibonacci");

    let base_of_fib: i32 = base_of_fib.trim()
        .parse()
        .expect("Failed to parsed string format");

    let result = fib(base_of_fib);

    println!("The result of fibonacci is: {}", result)
}

fn fib(number: i32) -> i32 {
    if number <= 0 {
        return 0;
    }

    if number == 1 {
        return 1;
    }

    // 0,1,1,2,3,5,8,13,21...

    return fib(number - 1) + fib(number - 2);
}
fn main() {
    let mut count = 0;
    // This is what we called label loop
    // Use this label loop when you want to `break` or `continue` in nested loop
    'counting_up: loop {
        println!("count, {count}");

        let mut remaining = 10;

        loop {
            println!("remaining: {remaining}");

            if remaining == 9 {
                break;
            } else if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }

        count += 1;
    };

    println!("End count = {count}");
}

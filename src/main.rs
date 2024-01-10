fn main() {
    // Multiple if else
    // let number = 6;

    // if number % 4 == 0 {
    //     println!("number is divisible by 4");
    // } else if number % 3 == 0 {
    //     println!("number is divisible by 3");
    // } else if number % 2 == 0 {
    //     println!("number is divisible by 2");
    // } else {
    //     println!("number is is not divisible by 2, 3, 4");
    // }

    // Using if in let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // This will break the code. since rust check the value of variable number is integer instead of string
    // let number = if condition { 5 } else { "six" };

    println!("The number is: {number}");
}

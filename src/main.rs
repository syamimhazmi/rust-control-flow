fn main() {
    // This code will be slow since compiler add runtime to check the index value.
    // let a = [10, 20, 30, 40, 50];

    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    // Another alternative to run this without compiler adding runtime,
    // we use for loop
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

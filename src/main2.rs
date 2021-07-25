// Learning Rust
//


// Handling Multiple Error Types - Rust by Example
// file:///home/infinite/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/rust-by-example/error/multiple_error_types.html
fn double_first(vec: Vec<&str>) -> i32 {
    let first = vec.first().unwrap(); // Generate error 1
    2 * first.parse::<i32>().unwrap() // Generate error 2
}

fn main() {
    // println!("Hello, world!");
    
    let numbers = vec!["42", "93", "18"];
    let empty = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("The first doubled is {}", double_first(numbers));

    // println!("The first doubled is {}", double_first(empty));
    // Error 1: the input vector is empty

    println!("The first doubled is {}", double_first(strings));
    // Error 2: the element doesn't parse to a number
}

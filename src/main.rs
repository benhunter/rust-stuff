fn main() {
    println!("Hello, rust!");

    // Rust Book
    // 4.2 References and Borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);  // Reference to s1, fucntion does not take ownership
    println!("The length of '{}' is {}.", s1, len);

    // Attempt to modify a borrowed value.
    change_borrowed(&s1);

    // Pass a mutable reference and modify it in a function.
    let mut s2 = String::from("hello");
    change(&mut s2);  // s2 must be declared as mutable
    println!("s2: {}", s2);

    // mutable references have one big restriction:
    // you can have only one mutable reference to a particular piece of data in a particular scope.
    let r1 = &mut s2;
    println!("r1: {}", r1);
    let r2 = &mut s2;
    // println!("r1: {}", r1);  // compile error here
    println!("r2: {}", r2);
 
    // create new scope to prevent data race
    let mut s3 = String::from("hello");
    {
        let r3 = &mut s3;
        println!("r3: {}", r3)
    } // r3 goes out of scope here
    let r4 = &mut s3;
    println!("r4: {}", r4)
}

fn calculate_length(s: &String) -> usize {
    // s is reference to a String
    // This is borrowing
    s.len()
    // when s goes out of scope, the reference is dropped
    // the original variable passed by reference is not dropped
    // because ownership did not change
}

fn change_borrowed(_some_string: &String) {
    // cannot borrow a reference as mutable
    // some_string.push_str(", world")  // uncomment to see compile error
}

fn change(some_string: &mut String) {
    some_string.push_str(", world")
}

#![allow(unused)]
fn main() {
    println!("Hello, rust!");

    // Rust Book
    // 4.1 What is Ownership?
    // Ownership Rules:
    // -Each value in Rust has a variable that’s called its owner.
    // -There can only be one owner at a time.
    // -When the owner goes out of scope, the value will be dropped.

    // The ownership of a variable follows the same pattern every time:
    // assigning a value to another variable moves it. When a variable
    // that includes data on the heap goes out of scope, the value will
    // be cleaned up by drop unless the data has been moved to be owned
    // by another variable.
    let s1 = String::from("hello");
    fn calculate_length_tuple(s: String) -> (String, usize) {
        let len = s.len();
        (s, len)
    }
    let (s2, len) = calculate_length_tuple(s1);
    println!("calculate_length_tulple(): length of '{}' is {}.", s2, len);

    // 4.2 References and Borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Reference to s1, function does not take ownership.
    println!("The length of '{}' is {}", s1, len);
    println!("{}", len);
    // Attempt to modify a borrowed value.
    change_borrowed(&s1);

    // Pass a mutable reference and modify it in a function.
    let mut s2 = String::from("hello");
    change(&mut s2); // s2 must be declared as mutable
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
    let r4 = &mut s3; // no race
    println!("r4: {}", r4);

    // Immutable references cannot exist while mutable references exist
    let mut s = String::from("hi hello");
    s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
                 // let r3 = &mut s; // BIG problem
                 // println!("{}, {}, and {}", r1, r2, r3); // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("{}, {}", r1, r2);

    // A reference's scope starts where it is introduced and ends after the last time it is used
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    let r3 = &mut s; // no problem
    println!("{}", r3);

    // Dangling References - a pointer that references a location in memory that may have been
    // given to someone else, by freeing some memory while preserving a pointer to that memory.
    // Rust compiler guarantees that references will never be dangling
    // let reference_to_nothing = dangle();
    let owner_of_string = no_dangle();
    println!("owner_of_string {}", owner_of_string);

    // Rules of References
    // -at any given time, you can have either one mutable reference or any number of immutable references
    // -references must always be valid

    // 4.3 Slice type, slices
    // Slice: another data type that does not have ownership
    // refer to a contiguous sequence of elements in a collection rather than the whole collection.
    // Slice is stored as a pointer to the starting element of the slice and a length.

    // Example problem: write a function that takes a string and returns the first word
    fn first_word_index(s: &String) -> usize {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return i;
            }
        }
        s.len()
    }

    fn test_first_word_index() {
        let s = String::from("one two");
        assert_eq!(3, first_word_index(&s));
    }
    test_first_word_index();

    fn first_word_slice(s: &String) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    fn test_first_word_slice() {
        let s = String::from("one two");
        assert_eq!(&s[0..3], first_word_slice(&s));
    }
    test_first_word_slice();


    // String Slices using range syntax.
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}", hello);
    println!("{}", world);

    // Slices with range syntax can omit index:
    let hello = &s[..5];
    let world = &s[6..];
    let hello_world = &s[..];
    // s.clear(); // Mutable reference (borrow) cannot be used before the immutable references (slices here)
    // go out of scope.
    println!("{} {} {}", hello, world, hello_world);

    // String literals are Slices! woah
    let s = "Hello, world!";
    // first_word_improved(&s);
    // test_first_word_improved();
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

// fn dangle() -> &String {
//    let s = String::from("hello");
//    &s
// } // because s is created inside dangle, s will be deallocated here. Solution is return the String directly.

fn no_dangle() -> String {
    let s = String::from("no_dangle");
    s
}

// A more experienced Rustacean would write:
// because it allows &String and &str values to be passed into the function.
fn first_word_improved(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s
}

#[test]
fn test_first_word_improved() {
    let my_string = String::from("hello world");
    let word = first_word_improved(&my_string);
    assert_eq!(word, "hello");
}


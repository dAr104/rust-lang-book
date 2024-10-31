use std::mem::take;

fn main() {
    // ----- Ownership rules -----
    // 1. Each value in Rust has a variable that's called its owner
    // 2. There can only be one owner at a time
    // 3. When the owner goese out of scope, the value will be dropped

    heap_and_stack();
    move_vs_shallow();

    let s = String::from("takes_onwership");
    takes_onwership(s);
    //println!("{}", s); //Error
    // passing a value to a function is a move operation, like assign the s1 to s2

    let x = 5;
    makes_copy(x);
    println!("{}", x); //No error

    let s1 = gives_ownership();
    println!("{}", s1); // gives_ownership moves the ownership from the function to s2

    let s2 = String::from("takes_and_gives_back");
    let s3 = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3); // takes_and_gives_back moves the ownership s2 into the function and then to s3

    // how doesn't move the ownership into a function and then back --> references
    let s4 = String::from("calculate_length");
    let length = calculate_length(&s4); // &s4 is a reference to s4, so no ownership is moved
    println!("The length of the string {} is: {}", s4, length);

    // passing a reference to a function is called "borrowing the value"

    let s5 = String::from("hello "); 
    // change(&s5); //Error
    let mut s6 = String::from("change the ");
    change(&mut s6); //No error, and the value of s6 is changed without moving the ownership

    // !!! you can have only one mutable reference to a particular piece of data in a particular scope
    let mut s7 = String::from("hello");
    let r1 = &mut s7;
    //let r2 = &mut s7; //Error, rust avoid data races at compile time
    
    // to fix this you can use immutable references
    let s7 = String::from("hello");
    let r1 = &s7;
    let r2 = &s7;
    println!("r1 = {}, r2 = {}", r1, r2);

    // what happens if a mix immutable and mutable references?
    let mut s8 = String::from("mix references");
    let r3 = &s8;
    //let r4 = &mut s8; //Error

    // you can't have a mutable reference while you have an immutable one in the same scope
    // scope start when the variable is declared and ends when the variable goes out of scope

    // dangling references: references to memory that has been deallocated
    let reference = dangle(); //Error

    /* The rule of references
    1) At any given time, you can have either one mutable reference or any 
    number of immutable references
    
    2) References must always be valid */

    // Slices
    // are references to a portion of a collection

    let mut str = String::from("hello world");
    let str2 = "hello world";

    let hello = &str[0..5]; // return hello
    let world = &str[6..11]; // return world
    let hello2 = &str[..5]; // return hello
    let world2 = &str[6..]; // return world
    // hello and world are two string slices

    let word = first_world(&str); // return 5
    let word2 = first_world_slices(str2); // return hello
    str.clear(); // error, str is borrowed as immutable


    // Slices can be used with arrays
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // return [2, 3]
}

fn heap_and_stack() {

    {
        let x = "hello"; // stack, string literal
    }
    
    {
        let y = String::from("hello"); 
        // the string hello will be store in the heap
        // and y in the stack, with pointer to the hello 
        // string in the heap
    }
    // rust handle the management of the heap memory for us
}

fn move_vs_shallow() {
    let x = 5;
    let y = x; //Copy
    // Copy is done for types that are stored in the stack, like scalar types

    let s1 = String::from("hello");
    let s2 = s1; //Move (not shallow copy)

    // Move means that the s1 is invalidated by rust to ensure memory safety

    //println!("{}, world!", s1); //Error
    println!("{}, world with move", s2);

    // To make a deep copy, we can use the clone method
    let s3 = s2.clone();
    println!("{}, world with clone", s3);
}

fn takes_onwership(some_string : String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer : u32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize { //s is a reference to a string
    let length = s.len();
    length
}

fn change(some_string: &mut String) {
    some_string.push_str("world"); //Error, reference are immutable by default
}

fn dangle() -> &String {
    let s = String::from("hello");
    &s //Error, s goes out of scope and the memory is deallocated
}

fn first_world(s: &String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_world_slices(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
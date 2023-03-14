pub fn ownership() {
    /*
                   OwnerShip rules
    1. Each value in Rust has a variable thats called it's owner.
    2. There can only be one owner at a time.
    3. When the owner get out of scope the will be droped.
    */

    {
        // s is not valid here, it's not yet declared
        let s = "hello"; // s is valid from this point forward
        println!("{}", s); // do stuff with s
    } // the scope is over now, and s is no longer valid

    ownership01();
    let s1 = gives_ownership();
    println!("My name is {}", s1);
    let s2 = String::from("hello");
    let s3 = takes_and_giveback_ownership(s2);
    println!("s1: {}, s3: {}", s1, s3);
    let (s4, len) = calculate_length(s3);
    println!("s4: {}, len: {}", s4, len);
    let leng = length(&s4);
    println!("s4: {}, len: {}", s4, leng);
    let mut s5 = String::from("hello");
    mutable_referances(&mut s5);
}

fn ownership01() {
    let x = 5;
    let _y = x; //copy

    let s1 = String::from("Hello");
    // let s2 = s1; // Move not shallow copy

    let _s2 = s1.clone();

    println!("{}, world!", s1);
    // takes_ownership(s1); // This will take the ownership
    takes_ownership(s1.clone()); // This will borrow the value
    println!("{}, world!", s1);
    makes_copy(x);
    println!("{}", x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(x: i32) {
    println!("{}", x);
}

fn gives_ownership() -> String {
    let s = String::from("Rohan");
    s
}

fn takes_and_giveback_ownership(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    return (s, length);
}

fn length(s: &String) -> usize {
    // Referances don't take ownership
    // s.push_str(" world"); // Referances are immutable by default
    let length = s.len();
    length
}

fn mutable_referances(s: &mut String) {
    s.push_str(" world");
    println!("{}", s);
}
fn multiple_mutable_referance() {
    let mut s1 = String::from("hello");

    let mut r1 = &mut s1;
    // cannot borrow `s1` as mutable more than once at a time, this feature reduce data races in Rust
    // let mut r2 = &mut s1;
    // println!("{},{}", r1, r2);

    let mut s2 = String::from("hello");

    let r3 = &s2;
    let r4 = &s2;
    // cannot borrow `s2` as mutable because it is also borrowed as immutable
    // let r5 = &mut s2;

    // println!("{},{},{}", r3, r4, r5);
}
fn multiple_imutable_referance() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    // cannot borrow `s2` as mutable because it is also borrowed as immutable
    // let r3 = &mut s;

    println!("{},{}", r1, r2);

    let r4 = &mut s;
}
/*
missing lifetime specifier, this function's return type contains a borrowed value, but there is no value for it to be borrowed
fn dangling_referances()-> &String{
    let s = String::from("hello");
    &s
}
*/

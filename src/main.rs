fn rust_loop() {
    let mut i = 0;
    let result = loop {
        print!("again!");
        i += 1;
        if i == 2 {
            // break; // Break the
            i += 5;
            break i; // Break and return value
        }
    };
    println!("RESULT: {}", result);
}
fn main() {
    // Imutable variable
    let x = 5;
    println!("The number is: {}", x);
    // x = x + 1;
    // println!("Number changed to:{}", x);

    //Mutable variable

    let mut y = 6;
    println!("y == {}", y);
    y = y + 1;
    println!("y+1 == {}", y);

    // Const
    const SUBSCRIBER_COUNT: u32 = 100_00;
    println!("{}", SUBSCRIBER_COUNT);

    //Shawoding
    shawoding();
    // Scaler dataType (single value )
    scalar_datatype();
    // Compound dataType (Group of value)
    compound_datatype();

    // Addition
    println!("{}", add(11, 12));
    println!("{}", subtract(22, 11));

    //control type
    control_flow();
    rust_loop();
    for_loop();
}

fn shawoding() {
    let z = 6;
    println!("z: {}", z);
    let z = "Hello";
    println!("z: {}", z);
}

fn scalar_datatype() {
    //1. Integer
    //2. Floating point numbers
    //3. Booleans
    //4. Charecter
}

fn compound_datatype() {
    //Tuple
    let tup = ("Let's get rusty", 100_100);
    // Getting value from Destructuring
    let (channel, subscriber) = tup;
    println!("{},{}", channel, subscriber);
    // Getting value Dot Notation
    println!("{},{}", tup.0, tup.1);

    // Array
    let error_codes = [200, 400, 500];
    let not_found = error_codes[1];

    let bytes = [0, 8]; // Array length is 8 and value of all element is 0

    println!("{}{}", not_found, bytes[0]);
}

fn add(x: i32, y: i32) -> i32 {
    let sum = x + y;
    return sum;
}
fn subtract(x: i32, y: i32) -> i32 {
    let sub = x - y;
    sub
}

fn control_flow() {
    let num = 5;
    if num < 10 {
        println!("first condition is true");
    } else if num < 22 {
        println!("second conndition is true");
    } else {
        println!("condition is false")
    }

    let condition = true;
    let number = if condition { 5 } else { 10 };
    println!("{}", number)
}

fn for_loop() {
    let a = [1, 2, 3, 4, 5, 6];

    for element in a {
        print!("{} ", element);
    }
    println!();
    for num in 100..105 {
        print!("{} ", num);
    }
}

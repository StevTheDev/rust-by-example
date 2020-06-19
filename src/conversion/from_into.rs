// The `From` and `Into` traits are inherently linked, and this is actually
// part of its implementation. If you are able to convert type A from type B,
// then it should be easy to believe that we should be able to convert type B 
// to type A.

// The `Into` trait is simply the reciprocal of the From trait. 
// If you have implemented the From trait for your type, `Into` will call it 
// when necessary.

// Using the `Into` trait will typically require specification of the type to
// convert into as the compiler is unable to determine this most of the time.
// However this is a small trade-off considering we get the functionality
// for free.

use std::convert::From;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

fn main() {
    let my_str = "hello";
    println!("my_str {:?}", my_str);
    let my_string = String::from(my_str);
    println!("my_string {:?}", my_string);

    let num1 = Number::from(30);
    println!("My number is {:?}", num1);

    let var = 5;
    // Try removing the type declaration
    let num2: Number = var.into();
    println!("My number is {:?}", num2);
}

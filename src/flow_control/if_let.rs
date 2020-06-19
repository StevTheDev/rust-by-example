#[allow(dead_code)]

fn main() {
    // All have type `Option<i32>`
    let number = Some(7);
    let none: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = none {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Did not match Some(i) (none is None)");
    }

    // More conditionals can be used to handle different outcomes

    let foo = false;

    if let Some(i) = none {
        println!("Matched {:?}!", i);
    // Destructure failed. Evaluate an `else if` condition to see if the
    // alternate failure branch should be taken:
    } else if foo {
        println!("Did not match Some(i) but foo == true");
    } else {
        // The condition evaluated false. This branch is the default:
        println!("Did not match Some(i) and foo == false");
    }


    // If let can be used with Enums too
    enum Color{
        Red,
        Green,
        Blue
    }
    let my_color = Color::Green;

    if let Color::Red = my_color {
        println!("Red!");
    } else if let Color::Green = my_color {
        println!("Green!");
    } else {
        println!("Blue!");
    }

    // This can be done in while loop conditions too

    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }
    // ^ `if let` had additional optional `else`/`else if`
    // clauses. `while let` does not have these.

    println!("optional: {:?}", optional)
}

fn main() {
    // Assign a reference of type `i32`. The `&` signifies there
    // is a reference being assigned.
    let value: i32 = 4;
    let reference = &value; // `reference` contains the memory address of `value`

    // To get the value of the obect at that memory address, `reference` can be
    // destructured or dereferenced.

    match reference { 
        // The compiler knows that `reference` is a memory address for an i32
        // In this match statement we can then match the pattern &x
        // This binds `value` to `x`, which we can use in this print statement
        &x => println!("Matched a reference. Destructured value: {:?}", x),
    }

    // A reference can be 'dereferenced' with *
    match *reference {
        // The compiler knows that `*reference` is an i32
        // In this match statement we can refer to the value of that object
        4 => println!("Got 4 by dereferencing"),
        val => println!("Got a value via dereferencing: {:}", val),
    }

    // What if you don't start with a reference? `reference` was a `&`
    // because the right side was already a reference. This is not
    // a reference because the right side is not one.
    let _not_a_reference = 3;

    // Rust provides `ref` for exactly this purpose. It modifies the
    // assignment so that a reference is created for the element; this
    // reference is assigned.
    let ref _is_a_reference = 3;

    // Accordingly, by defining 2 values without references, references
    // can be retrieved via `ref` and `ref mut`.
    let value = 5;
    let mut mut_value = 6;

    // Use `ref` keyword to create a reference.
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    // Use `ref mut` similarly.
    match mut_value {
        ref mut m => {
            // Got a reference. Gotta dereference it before we can
            // add anything to it.
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

    println!("mut_value: {:?}", mut_value);

    // Keywork ref  can be used in place of &

    // Repeat of first excercise using ref instead of &
    let value: i32 = 4;
    let ref reference = value; // `reference` contains the memory address of `value`

    // To get the value of the obect at that memory address, `reference` can be
    // destructured or dereferenced.

    match reference { 
        // The compiler knows that `reference` is a memory address for an i32
        // In this match statement we can then match the pattern &x
        // This binds `value` to `x`, which we can use in this print statement
        &x => println!("Matched a reference. Destructured value: {:?}", x),
    }

    // A reference can be 'dereferenced' with *
    match *reference {
        // The compiler knows that `*reference` is an i32
        // In this match statement we can refer to the value of that object
        4 => println!("Got 4 by dereferencing"),
        val => println!("Got a value via dereferencing: {:}", val),
    }

}

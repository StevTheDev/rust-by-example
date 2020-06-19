fn main() {
    // This binding lives in the main function
    let long_lived_binding = 1;
    let mut mutable_binding = "Hello";
    println!("Scope: Main");
    println!("long_lived_binding: {}", long_lived_binding);

    // This is a block, and has a smaller scope than the main function
    {
        println!("Scope: Block");
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("short_lived_binding: {}", short_lived_binding);
        println!("long_lived_binding: {}", long_lived_binding);
        println!("mutable_binding: {}", mutable_binding);


        // long_lived_binding can be changed witin this scope. "Shadowing"
        let long_lived_binding: f32 = 5.5;
        println!("shadowed long_lived_binding: {}", long_lived_binding);

        // Changes to the mutable variable in this scope will persist in the outer scope
        println!("mutable_binding: {}", mutable_binding);
        mutable_binding = "World";
        println!("mutable_binding: {}", mutable_binding);

        // When shadowing a mutable variable, omitting mut from the declration will "freeze" the variable within the scope
        let mutable_binding = mutable_binding;
        // mutable_binding = "Hello World";
        // println!("changed frozen mutable_binding: {}", mutable_binding);

    }
    println!("Scope: Main");
    // End of the block
    // `short_lived_binding` is no longer usable

    println!("long_lived_binding: {}", long_lived_binding);
    
    // This binding also *shadows* the previous binding
    let long_lived_binding = 'a';
    
    println!("shadowed long_lived_binding: {}", long_lived_binding);
    println!("mutable_binding: {}", mutable_binding);

    // Declare a variable binding
    let a_binding;

    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    let another_binding;

    // Error! Can't use until initialized
    // println!("another binding: {}", another_binding);

    another_binding = 1;

    println!("another binding: {}", another_binding);

}

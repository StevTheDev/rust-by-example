fn main() {
    let number = 13;
    // TODO ^ Try different values for `number`

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }

    let boolean = true;
    // Match is an expression too
    let binary = match boolean {
        // The arms of a match must cover all the possible values
        false => 0,
        true => 1,
        // TODO ^ Try commenting out one of these arms
    };

    println!("{} -> {}", boolean, binary);


    // Match statements can use more conditional checks called "guards" 
    let point = (-2,0);

    match point {
        (x, y) if x == 0 && y == 0 => println!("Point ({},{}) at the origin", x, y),
        (x, y) if x > 0 && y > 0 => println!("Point ({},{}) is in Q1", x, y),
        (x, y) if x > 0 && y < 0 => println!("Point ({},{}) is in Q2", x, y),
        (x, y) if x < 0 && y < 0 => println!("Point ({},{}) is in Q3", x, y),
        (x, y) if x < 0 && y > 0 => println!("Point ({},{}) is in Q4", x, y),
        (_, _) => println!("Point is on an axis"),   
    }


    // You can match on a function result using @ and providing values to match against

    fn age() -> u32 {
        40
    }

    match age() {
        0 => println!("Newborn"),
        n @ 1 ..=12 => println!("Child of age {}", n),
        n @ 13 ..= 20 => println!("Teen of age {}", n),
        n if n < 35 => println!("Adult of age {}", n),
        _ => println!("Old enough to be President"),
    }

    // Function results can be destructured as well
    match age() {
        42 => println!("42!"),
        age => println!("{}!", age),
        _ => (),
    }
}

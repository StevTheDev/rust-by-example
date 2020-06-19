// for - loop over iterators

fn main() {
    // Range Notation:
    // 1..101 is Exclusive (1-100)
    // 1..=101 is Inclusive (1-101)
    for n in 1..=15 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // The for in construct is able to interact with an Iterator in 
    // several ways. As discussed in the section on the Iterator trait, 
    // by default the for loop will apply the into_iter function to the 
    // collection. However, this is not the only means of converting 
    // collections into iterators.
    let mut names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    // iter_mut - This mutably borrows each element of the collection, 
    // allowing for the collection to be modified in place.

    for name in names.iter_mut() {
        *name =match name{
            &mut "Ferris" => name,
            _ => "Ferris",
        }
    }

    // into_iter - This consumes the collection so that on each iteration 
    // the exact data is provided. Once the collection has been consumed 
    // it is no longer available for reuse as it has been 'moved' within 
    // the loop. This means that 'names' no longer owns the vector object

    for name in names.into_iter() {
        println!("Hello {}", name);
    }
    println!("There are rustaceans among us!");
    // Using 'names' is no longer allowed as it does not own anything
    // println!("{} names remaining.", names)

    names = vec!["Alice", "Joe", "Ferris"];
    println!("{} names.", names.len())

}
 
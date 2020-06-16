use std::fmt;

// Derive the `fmt::Debug` implementation for `Structure`. `Structure`
// is a structure which contains a single `i32`.
#[derive(Debug)]
struct Structure(i32);

// Implement the Display trait for struct Structure
impl fmt::Display for Structure {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}

// Put a `Structure` inside of the structure `Deep`. Make it printable
// also.
#[derive(Debug)]
struct Deep(Structure);

impl fmt::Display for Deep {
    // Display for the 'Deep' struct will leverage the trait on Structure 
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0) // self.0 is a Structure
    }
}

// A more complex structure
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

impl fmt::Display for Person<'_> { // <'a> and <'_> mark 'lifetime`
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{name} ({age})", name=self.name, age=self.age)
    }
}

// A structure holding two numbers. `Debug` will be derived so the results can
// be contrasted with `Display`.
#[derive(Debug)]
struct MinMax(i64, i64);

// Implement `Display` for `MinMax`.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// Similarly, implement `Display` for `Point2D`
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

impl fmt::Binary for Point2D {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x_int = self.x.trunc() as i32;
        let y_int = self.y.trunc() as i32;

        let x_dec: i64 = self.x.fract().to_string().replace("0.","").parse().unwrap();
        let y_dec: i64 = self.y.fract().to_string().replace("0.","").parse().unwrap();

        write!(f, "x: {x_int:b}.{x_dec:b}, y: {y_int:b}.{y_dec:b}", x_int=x_int, x_dec=x_dec, y_int=y_int, y_dec=y_dec)
    }
}

// More Complex Structure
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f, "[")?;

        // Iterate over `v` in `vec` while enumerating the iteration
        // count in `count`.
        for (i, v) in vec.iter().enumerate() {
            // For every element except the first, add a comma.
            // Use the ? operator, or try!, to return on errors.
            if i > 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
            // write!(f, "{0}: {1}", i, v)?; // Prints [Index: Value, ...]
        }

        // Close the opened bracket and return a fmt::Result value.
        write!(f, "]")
    }
}

struct City {
    name: &'static str,
    lat: f32,
    lon: f32,
}

impl fmt::Display for City {
    // `f` is a buffer, and this method must write the formatted string into it
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
        // `write!` is like `format!`, but it will write into a buffer (the first argument)
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let hex = format!("{0:02x}{1:02x}{2:02x}", self.red, self.green, self.blue);
        write!(f, "rgb({red}, {green}, {blue}) #{hex}", red=self.red, green=self.green, blue=self.blue, hex=hex)
    }
}

fn main() {
    // Printing with `{:?}` is similar to with `{}`.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    println!("Display vs Debug Formatting");

    // `Structure` is printable with {} using the Dsiplay trait 
    println!("Display: {}", Structure(3));
    println!("Display: {}", Deep(Structure(7)));
    
    // The problem with `derive` debug is there is no control over how
    // the results look.
    println!("Debug: {:?}", Structure(3));
    // Using # for pretty printing
    println!("Debug: {:#?}", Deep(Structure(7)));
    
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };
    println!("Printing a `Person` {}", peter);

    let minmax = MinMax(0, 14);
    println!("Printing a `MinMax`");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 1523.3, y: 7231.2 };

    println!("Printing a `Point2D`");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);
    println!("Binary: {:b}", point);

    println!("Printing a `List`");
    let v = List(vec![1, 2, 3]);
    println!("{}", v);


    println!("Printing `Cities`");
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ].iter() {
        // Switch this to use {} once you've added an implementation
        // for fmt::Display.
        println!("Color Debug: {:?}", *color);
        println!("Color Display: {}", *color);
    }
}

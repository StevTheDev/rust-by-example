use std::fmt;

#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        writeln!(f, "top_left: ({0}, {1})", self.top_left.x, self.top_left.y);
        writeln!(f, "bottom_right: ({0}, {1})", self.bottom_right.x, self.bottom_right.y);
        Ok(())
    }
}

fn area(rect: Rectangle) -> f32 {
    let height = rect.top_left.y - rect.bottom_right.y;
    let width = rect.bottom_right.x - rect.top_left.x;
    return (width * height).abs()
}

fn square(bottom_left: Point, length: f32) -> Rectangle {
    // Return a Rectangle which is a square with sides of length

    let top_right: Point = Point {
        x: (bottom_left.x + length),
        y: (bottom_left.y + length)
    };
    
    return Rectangle {
        top_left: Point {
            x: bottom_left.x,
            y: top_right.y
        },
        bottom_right: Point {
            x: top_right.x,
            y: bottom_left.y
        }
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let top_left: Point = Point { x: 5.0, y: 1.0 };

    // Access the fields of the point
    println!("top left: ({}, {})", top_left.x, top_left.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 10.0, y: 0.0 };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("bottom right: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = top_left;

    println!("top_edge {top_edge} left_edge {left_edge}",top_edge=top_edge,left_edge=left_edge);

    let rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: top_left,
        bottom_right: bottom_right,
    };

    println!("Rectangle: {}",rectangle);
    println!("Rectangle area is {}", area(rectangle));

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let rectangle = square(Point { x: 0.0, y: 0.0 }, 2.0);
    println!("Rectangle: {}",rectangle);
    println!("Rectangle area is {}", area(rectangle));

}

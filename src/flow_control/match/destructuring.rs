#[allow(unused_variables)]
#[allow(unreachable_patterns)]

fn main() {
    let tuple = (1, 5);
    // TODO ^ Try different values for `pair`

    println!("tuple: {:?}", tuple);
    // Match can be used to destructure a tuple
    match tuple {
        // Destructure the second
        (0, y) => println!("First is `0` and `y` is `{:?}`", y),
        (x, 0) => println!("`x` is `{:?}` and last is `0`", x),
        (x, y) => println!("`x` is `{:?}` and `y` is `{:?}`", x, y),
        // _      => println!("It doesn't matter what they are"),
        // `_` means don't bind the value to a variable
    }

    // x and y are not available in the outer scope
    // println!("`x` is `{:?}` and `y` is `{:?}`", x, y);

    // Destructuring Enums

    enum Color {
        // These 3 are specified solely by their name.
        Red,
        Blue,
        Green,
        // These likewise tie `u32` tuples to different names: color models.
        RGB(u32, u32, u32),
        HSV(u32, u32, u32),
        HSL(u32, u32, u32),
        CMY(u32, u32, u32),
        CMYK(u32, u32, u32, u32),
    }

    let red = Color::Red;
    let green = Color::Green;
    let blue = Color::Blue;
    let rgb = Color::RGB(122, 17, 40);
    let hsv = Color::HSV(0, 50, 50);
    let hsl = Color::HSL(122, 17, 40);
    let cmy = Color::CMY(122, 17, 40);
    let cmyk = Color::CMYK(122, 17, 40, 10);
    // TODO ^ Try different variants for `color`

    println!("What color is it?");
    // An `enum` can be destructured using a `match`.
    match hsv {
        Color::Red   => println!("The color is Red!"),
        Color::Blue  => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) =>
            println!("Red: {}, green: {}, and blue: {}", r, g, b),
        Color::HSV(h, s, v) =>
            println!("Hue: {}, saturation: {}, value: {}", h, s, v),
        Color::HSL(h, s, l) =>
            println!("Hue: {}, saturation: {}, lightness: {}", h, s, l),
        Color::CMY(c, m, y) =>
            println!("Cyan: {}, magenta: {}, yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, magenta: {}, yellow: {}, key (black): {}",
                c, m, y, k),
        // Don't need another arm because all variants have been examined
    }

    // Destructuring Structs
    println!("Destructuring Structs");

    struct Event {
        point: (u32, u32),
        scale: u32,
    }

    // Try changing the values in the struct to see what happens
    let foo = Event {
        point: (1, 2),
        scale: 3
    };

    match foo {
        Event { point: (0, 0), scale } => println!("{}x Event at origin" , scale),

        // the order of struct members is not important
        Event { scale: s, point: p } => println!("{}x Event at ({}, {})", s, p.0, p.1),

        // and you can also ignore some variables:
        Event { scale, .. } => println!("{}x Event happened somewhere", scale),
        // this will give an error: pattern does not mention field `x`
        //Foo { y } => println!("y = {}", y);
    }

}

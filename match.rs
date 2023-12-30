fn main() {
    let a = 1;

    match a {
        1 => println!("Matched"),
        _ => println!("No Match"),
    }

    //matching enum
    enum Color {
        Red,
        Green,
        Blue,
    }

    let my_color = Color::Green;

    // use of match expression to match against an enum variant
    match my_color {
        Color::Red => println!("The color is red"),
        Color::Green => println!("The color is green"),
        Color::Blue => println!("The color is blue"),
    }
}
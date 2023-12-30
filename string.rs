// String::from() method

fn main() {
    let mut x = String::from("Hello World");

    println!("{}",x);

    let slice = &x[0..2];
    println!("sliced={}",slice);

    for char in x.chars() {
        println!("{}", char)
    }

    x.push__str("yz"); // append
}
// way to organize larger codebases
// encapsulate functionality with pub && priv, expose only what is necessary
// helpful to access code across multiple files, ex: main.rs & math.rs


// In main.rs
use arithmetic::{add, subtract};

fn main() {
    let sum = add(5, 3);
    let difference = subtract(8, 2);
    println!("Sum: {}, Difference: {}", sum, difference);
}

// In another file named math.rs
pub mod arithmetic {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}
use std::io;

fn main() {
println!("Enter a string:");
let mut user_input = String::new();

io::stdin().read_line(&mut user_input).expect("failed to read");
let user_input = user_input.trim();

let reverse_string = reverse(user_input);
println!("{}", reverse_string);
}

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

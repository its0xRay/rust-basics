// group related pieces of data together

use std::io;

struct person {
    name: String,
    age: u8,
    score: f64
}

enum EmployementStatus {
    employed,
    unemployed,
    fafo
}

fn get_status() -> EmployementStatus {
    println!("Enter your employement status");
    let mut input = String::new();

    io::stdin().readline(&mut input).expect("Failed to read line");

    match input.trim().to_lowercase().as_str() {
        "employed" => EmployementStatus::employed,
        "unemployed" => EmployementStatus::unemployed,
        "fafo" => EmployementStatus::fafo,
        _ => {
            println!("Enter a valid status")
        }
    }
}

fn main() {
    
    let p = person {
        name: String::from("0xRay"),
        age: 100,
        score: 1.81
    };

    println!("Name is ={}", p.name);
    println!("Age: {}",p.age);

    let EmployementStatus = get_status();

    match EmployementStatus {
        EmployementStatus::employed => println!("employed"),
        EmployementStatus::unemployed => println!("unemployed"),
        EmployementStatus::fafo => println!("fafo"),
    }
}
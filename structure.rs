fn main() {
    struct person {
        name: String,
        age: u8,
        height: u8
    }

    let p = person {
        name: String::from("0xRay"),
        age: 100,
        height: 180
    }

    println!("Name is ={}", p.name);
}
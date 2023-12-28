fn main() {
    let num1:i32=10;
    let num2:i32=20;
    add(num1,num2);
    println!("sub of 2 numbers is ={}", sub(num1,num2));
    println!("mul of 2 numbers is ={}", mul(num1,num2));
}
 
fn add(num1:i32,num2:i32) {
    println!("total is {}",(num1+num2));
}

fn sub(num1:i32, num2:i32) -> i32 {
    return num1-num2;
}

fn mul(num1:i32, num2:i32) -> i32 {
    num1*num2  // expression, return keyword not needed
}
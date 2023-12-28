fn main(){
    let x:i32=10;
    if x>5 {
        println!("X is greater than 5")
    } else
    {
        println!("X is less than 5")
    }
    let result= if x>10{
        return x+100
    } else {
        return x-10
    }
    println!("Result is {result}")
}

fn main() {
    let x =10;
    let y = 20;

    println!("{}", add(x,y));
    println!("{}", sub(x,y));
    println!("{}", mul(x,y));
    println!("{}", div(x,y));
    println!("{}", moddd(x,y));
}
fn add(x: i32, y:i32) -> i32{
    x + y
}

fn sub(x: i32, y:i32) -> i32{
    x - y
}

fn mul(x: i32, y:i32) -> i32{
    x * y
}

fn div(x: i32, y:i32) -> i32{
    x / y
}

fn moddd(x: i32, y:i32) -> i32{
    x % y
}
fn main() {
    let x =10;
    let y = 20;

    println!("{}", add(x,y));
    println!("{}", sub(x,y));
    println!("{}", mul(x,y));
    println!("{}", div(x,y));
    println!("{}", moddd(x,y));

    //조건문
    let number = 20;
    if number > 5 {
        println!("number is bigger then 5");
    }
    else if number == 5 {
        println!("number is same with 5");
    }
    else{
        println!("number is less than 5");
    }

    //반복문
    let numbers: [i32; 5] = [1,2,3,4,5];
    for number in numbers.iter() {
        println!("The number is : {}",number);
    }

    let mut count: i32 = 0;
    loop {
        count += 1;
        println!("count is {}",count);
        if count == 10 {
            break;
        }
    }

    while count > 2 {
        println!("count is {}",count);
        count -= 1;
    }
    println!("count is {}",count);


    for number in 1..=10 {
        println!("number is {}",number);
    }

    
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
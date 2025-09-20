fn main() {
    let sum = sum(9,8);
    println!("{}",sum);
    let rest = rest(10,7);
    println!("{}", rest);
    let divide = divi(10.0,5.0);
    println!("{}", divide);
    let multiplication = multiplication(20.0,5.0);
    println!("{}", multiplication);
    println!("try!");
}

fn sum(number1: i8,number2:i8)-> i8{
    number1 + number2
    
}

fn rest(number1 : i8, number2: i8) -> i8
{
    number1 - number2
}

fn divi(number1 : f32, number2 : f32) -> f32
{
   number1 / number2
}


fn multiplication(number1 : f32, number2 : f32) -> f32
{
    number1 * number2
}

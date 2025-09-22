use std::io;

fn main(){

    loop {
        println!("Hello Im working with Rust, today Will be doing a Calculator");
        println!("Option 1: Sum");
        println!("Option 2: Rest");
        println!("Option 3: Multiplication");
        println!("Option 4: Dividation");
        println!("Option 5: Square");
        println!("Option 6: Square root");
        println!("What is your choice?");
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to ReadLine");
        
        let choice : i32 = match choice.trim().parse() 
        {
            Ok(num) => num,
            Err(_) => {
                println!("Please put a real number");
                continue;
            }

        };
        println!("You chose: {}", choice);


        println!("Enter first number:");
        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to ReadLine");

        println!("Enter second number:");
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to ReadLine");

        
        
        let num1 : f32 = match input1.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("Please put a real number");
                continue;
            }

        };

        
        let num2 : f32 = match input2.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("Please put a real number");
                continue;
            }
        };

        
        if choice == 0
        {
            break;
        }
        else if choice == 1
        {
            let total_sum = sum(num1, num2);
            println!("Total Sum {}", total_sum);   
        }
        else if choice == 2
        {
            let total_rest = rest(num1,num2);
            println!("Total Rest {}", total_rest);   
        }else if choice == 3
        {
            let total_multiplication = mult(num1, num2);
            println!("Total Multiplication {}", total_multiplication);   
        }else if choice == 4
        {
            let total_dividation = div(num1,num2);
            println!("Total Dividation {}", total_dividation);   
        }else if choice == 5
        {
            let total_square = square(num1);
            println!("Total Square Root {}", total_square);   
        }else if choice == 6
        {
            let total_square_root = square_root(num1);
            println!("Total Square Root {}", total_square_root);   
        }

    }
}

fn sum(number1 : f32, number2: f32) -> f32
{
    number1 + number2
    
}


fn rest(number1: f32, number2: f32) -> f32
{
    number1 - number2
}

fn mult(number1: f32, number2:f32) -> f32
{
    number1 * number2
}

fn div(number1: f32, number2:f32) -> f32
{
    number1 / number2
}

fn square(num1: f32) -> f32
{
    let operation = num1.powi(2);
    return operation;

}

fn square_root(num1:f32) -> f32
{
    let operation = num1.sqrt();
    return operation;
}
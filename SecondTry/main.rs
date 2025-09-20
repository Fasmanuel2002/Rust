use std::io;

fn main(){

    loop {
        println!("Hello Im working with Rust, today Will be doing a Calculator");
        println!("Option 1: Sum");
        println!("Option 2: Rest");
        println!("Option 3: Multiplication");
        println!("Option 4: Dividation");
        println!("Option 5: Square");
        println!("Option 6: square root");
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


        let mut input1 = String::new();
        io::stdin().read_line(&mut input1).expect("Failed to ReadLine");

        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("Failed to ReadLine");

        
        println!("Choose the first number: ");
        let num1 : f32 = match input1.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("Please put a real number");
                continue;
            }

        };

        println!("Choose the Second number: ");
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

    }
}

fn sum(number1 : f32, number2: f32) -> f32
{
    number1 + number2
    
}
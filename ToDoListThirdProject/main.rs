use std::io;

#[derive(Debug)]
struct Task{
    description: String,
    completed:bool
}

fn main()
{

    //Creating task 
    let mut tasks: Vec<Task> = Vec::new();

    loop 
    {
        println!("Hello Im working with Rust, today will be doing a to List");
        println!("Option 1: Add Task");
        println!("Option 2: List of Task");
        println!("Option 3: Mark as Done");
        println!("Option 4: Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read Line");

        
        let choice: i32 = match choice.trim().parse()
        {
            Ok(num) => num,
            Err(_) => {
                println!("Please put a number");
                continue;
            }

        };
        println!("Choose a number please: {}", choice);

        match choice 
        {
            1 =>
            {
                println!("Enter the task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read Line");
                add_task(&mut tasks, description.trim().to_string());
            },
            2 =>
            {

                read_task(&tasks);

            }
            _ => break,

        };


    }

}

fn add_task(tasks: &mut Vec<Task>, description: String)
{
    let task = Task{
        description,
        completed: false,
    };   
    tasks.push(task);
}
fn read_task(tasks: &Vec<Task>)
{
    for task in tasks{
        println!("Task {:?}", task);
    }

}
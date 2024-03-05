use rand::Rng;
use std::io;

fn main() {
    let mut answer: bool = true;
    let mut input: String = String::new();
    let mut counter = 0;

    while true {

        println!("Do you want to continue? y/n ");
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim().to_lowercase();
        let answer = match input.as_str() {
            "y" => true,
            "n" => false,
            _ => {
                println!("Invalid input. Please enter y or n.");
                return;
            }
        };
        if answer == false {
            break;
        }

        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(2..=10);

        counter += random_number;
        println!("Currently, you've got {counter} points! ")
    }
}
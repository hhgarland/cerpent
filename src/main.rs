use std::env;

fn main() {
    // Check if a number argument is provided
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./percentage_calculator <number>");
        std::process::exit(1);
    }

    // Parse the input number
    let input_number = match args[1].parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Error: Invalid number provided.");
            std::process::exit(1);
        }
    };

    // Calculate and print a range of percentages in increments of 5%
    for percentage in (1..=20).step_by(1) {
        let result = (input_number as f64 * (percentage * 5) as f64) / 100.0;
        println!("{}% of {} is {:.2}", percentage * 5, input_number, result);
    }
}


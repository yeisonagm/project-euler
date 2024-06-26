use std::env;
use std::fs;
use std::io;
use std::io::Write;

mod math_operations;
mod problems;

use problems::*;


fn read_files() -> Vec<String> {
    let mut problems: Vec<String> = Vec::new();
    let mut dir = String::new();

    // Get current working directory
    if let Ok(current_dir) = env::current_dir() {
        dir.push_str(current_dir.to_str().expect("Directory error"));
    }
    dir.push_str("/src/problems");
    println!("Path to directory: {}\n", dir);

    // It reads all the problem files and adds it to the vector
    if let Ok(paths) = fs::read_dir(dir) {
        for path in paths {
            let filename = path.unwrap().file_name().into_string().unwrap();
            if filename.starts_with('p') {
                // Add only the problems and format the name of each one
                problems.push(filename[1..filename.len() - 3].to_string().replace("_", " "));
            }
        }
    }

    problems
}

fn show_menu(problems: &Vec<String>) -> u16 {
    println!("List of problems ({})", problems.len());
    problems.into_iter().for_each(|e| println!("\t{}", e));
    println!("");

    let mut option: i16 = 0;

    while option < 1 || option > problems.len() as i16 {
        let mut line = String::new();
        if option <= 0 || option > problems.len() as i16 {
            print!("Problem to run: ");
        }

        // Ensures that the message is displayed before the program waits for user input
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        option = line.trim().parse().unwrap_or(0);
    }

    option as u16
}

fn run_problem(problem: u16) {
match problem {
        1 => p001_multiples_of_3_or_5::solve(),
        2 => p002_even_fibonacci_numbers::solve(),
        3 => p003_largest_prime_factor::solve(),
        4 => p004_largest_palindrome_product::solve(),
        5 => p005_smallest_multiple::solve(),
        6 => p006_sum_square_difference::solve(),
        _ => println!("Choose a fixed issue to run"),
    }
}


fn main() {
    let mut problems = read_files();
    problems.sort();

    let problem = show_menu(&problems);
    run_problem(problem)
}

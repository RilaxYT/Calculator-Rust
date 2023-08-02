use std::io;
use std::thread;
use std::time::Duration;
use std::io::{stdout, Write};
use crossterm::{execute, terminal::{Clear, ClearType}, cursor::MoveTo};
use crossterm::style::Stylize;

enum Command {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    SquareRoot,
    Help,
    Clear,
    Exit,
    Invalid
}

trait RoundTo {
    fn round_to(&self, decimal_places: u32) -> f64;
}

impl RoundTo for f64 {
    fn round_to(&self, decimal_places: u32) -> f64 {
        let multiplier = 10.0_f64.powi(decimal_places as i32);
        (self * multiplier).round() / multiplier
    }
}

fn clear_console() {
    let mut stdout = stdout();

    execute!(stdout, Clear(ClearType::All), MoveTo(0, 0)).unwrap();
    stdout.flush().unwrap();
}

fn help() {
    println!();
    println!("[ {} | {} | {} | {} | {} | {} | {} ]", 
            "+".green(), "-".green(), "*".green(), "/".green(), "%".green(), "^".green(), "sqrt".green());
    println!("{} - show this message", "help".green());
    println!("{} or {} - cleans the console", "clear".green(), "cls".green());
    println!("{} - exit the program", "exit".green());
    println!();
}

fn main() {
    let ver: &str = "v1.1.0a_02";
    println!("{} {}", "Kalkulator RUST".cyan(), ver.red());
    help();

    let mut exit = false;
    while !exit {
        let mut exit_input = false;
        let mut no_op = false;

        let mut a: f64 = 0.;
        let mut b: f64 = 0.;
        let out: f64;

        print!("> ");
        io::stdout().flush().unwrap();
        let mut com = String::new();
        io::stdin()
            .read_line(&mut com)
            .unwrap_or_else(|_| panic!("{}", "Error: failed to read line!".dark_red()));
        com = com.trim().to_string().to_lowercase(); // Remove the newline from the input and change it to lower case

        let op = match com.as_str() {
            "+" => Command::Add,
            "-" => Command::Subtract,
            "*" => Command::Multiply,
            "/" => Command::Divide,
            "%" => Command::Modulo,
            "^" => Command::Power,
            "sqrt" => Command::SquareRoot,
            "help" => {
                help();
                no_op = true; 
                Command::Help
            },
            "clear" => { 
                clear_console(); 
                help();
                no_op = true;
                Command::Clear 
            },
            "cls" => { 
                clear_console(); 
                help();
                no_op = true;
                Command::Clear 
            },
            "exit" => {
                exit = true;
                no_op = true;
                Command::Exit
            },
            _ => {
                println!("{}", "Error: invalid command!".dark_red());
                no_op = true;
                Command::Invalid
            }
        };

        if !no_op {
            let mut action_nr: bool = false;
            while !exit_input {
                if !action_nr {
                    print!("Number A> ")
                } else {
                    print!("Number B> ");
                    exit_input = true
                }
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .unwrap_or_else(|_| panic!("{}", "Error: failed to read line!".dark_red()));
                input = input.trim().to_string(); // Remove the newline character from the input

                if !valid_characters(&input) || input.is_empty() {
                    println!("{}", "Error: invalid characters!".dark_red());
                    input = "0".to_owned()
                }

                if !action_nr {
                    a = input.trim().parse().unwrap();
                    action_nr = true
                } else {
                    b = input.trim().parse().unwrap();
                    action_nr = false
                }
            }

            out = arithmetic_unit(op, a, b).round_to(3);
            println!();
            println!("{} {}", "Result:".white(), out);
            println!();
        }
    }
    println!("Bye!");
    thread::sleep(Duration::from_millis(400));
}

fn arithmetic_unit(op: Command, a: f64, b: f64) -> f64 {
    match op {
        Command::Add => a + b,
        Command::Subtract => a - b,
        Command::Multiply => a * b,
        Command::Divide => {
            if b == 0. {
                println!("{}", "Error: division by zero!".dark_red());
                0.
            } else {
                a / b
            }
        },
        Command::Modulo => {
            if b == 0. {
                a
            } else {
                a % b
            }
        },
        Command::Power => a.powf(b),
        Command::SquareRoot => {
            // b is the degree 
            if b == 0. {
                a.powf(1. / 2.)
            } else {
                a.powf(1. / b)
            }
        
        },
        _ => 0. // this should never happen
    }
}

fn valid_characters(my_string: &str) -> bool {
    let mut count_dot: u32 = 0;
    let mut count_minus: u32 = 0;

    let is_valid = my_string.chars().all(|c| {
        if c == '.' {
            count_dot += 1;
            count_dot <= 1 // One dot at most
        } else if c == '-' {
            count_minus += 1;
            count_minus <= 1 // One minus sign at most
        } else {
            c.is_ascii_digit() // Other characters must be numbers
        }
    });

    is_valid
}
// use chapeter2::value_to_color_string;
use chapeter2::evens;
use rand::Rng;
use std::cmp::Ordering;
use std::ops::Add;
use std::{io, result};

use crate::caculate::{can_construct_note, evaluate, CalculatorInput};

mod caculate;

fn main() {
    // println!("Guess the number!");
    // let secret_number = rand::thread_rng().gen_range(1..101);

    // loop {
    //     println!("Please input your guess.");

    //     let mut guess = String::new();

    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     println!("You guessed: {}", guess);

    //     match guess.cmp(&secret_number) {
    //         Ordering::Less => println!("Too small"),
    //         Ordering::Greater => println!("Too big"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }

    // fn test(speed: u8) -> f64 {
    //     if speed <= 4 {
    //         let speed = speed as f64;
    //         let result = speed * 221 as f64;
    //         return result;
    //     } else if speed >= 5 && speed <= 8 {
    //         let speed = speed as f64;
    //         let result = speed * 221 as f64 * 0.9;
    //         return result;
    //     } else {
    //         let speed = speed as f64;
    //         let result = speed * 221 as f64 * 0.7;
    //         return result;
    //     }
    // }

    pub fn working_items_per_minute(speed: u8) -> u32 {
        if speed <= 4 {
            let result = (speed * 221 % 60) as u32;
            return result;
        } else if speed >= 5 && speed <= 8 {
            let result = (speed as f64 * 221 as f64 * 0.9 / 60 as f64) as u32;
            return result;
        } else {
            let result = (speed as f64 * 221 as f64 * 0.77 / 60 as f64) as u32;
            return result;
        }
    }

    // println!("result is {}", working_items_per_minute(10));

    pub enum LogLevel {
        Info,
        Warning,
        Error,
    }

    pub fn log(level: LogLevel, message: &str) -> String {
        match level {
            LogLevel::Error => {
                let pre = String::from("[ERROR]: ");
                let result = pre.add(message);
                return result;
            }
            LogLevel::Info => {
                let pre = String::from("[INFO]: ");
                let result = pre.add(message);
                return result;
            }
            LogLevel::Warning => {
                let pre = String::from("[WARNING]: ");
                let result = pre.add(message);
                return result;
            }
        }
    }

    // println!("{}", log(LogLevel::Warning, "what the fuck"))
    // value_to_color_string(1)
    // let mut iter = vec![1, 2, 3];
    // iter.push(4);
    // println!("{}", iter.pop().unwrap())
    // let eveniter = evens(iter);

    let inputs = [
        CalculatorInput::Value(4),
        CalculatorInput::Value(8),
        CalculatorInput::Add,
        CalculatorInput::Value(7),
        CalculatorInput::Value(5),
        CalculatorInput::Subtract,
        CalculatorInput::Multiply,
    ];
    // println!("{}", evaluate(&inputs).unwrap())

    let magazine = "two times three is not four"
        .split_whitespace()
        .collect::<Vec<&str>>();
    let note = "two times two is four"
        .split_whitespace()
        .collect::<Vec<&str>>();
    println!("{}", can_construct_note(&magazine, &note));
}

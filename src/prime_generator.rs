use core::panic;

use rand::Rng;
use rug::{integer::IsPrime, Assign, Integer};

pub fn genarator(args: Vec<String>) -> Integer {
    let mut digits: u32 = 10;
    let mut reps: u32 = 20;
    let mut args = args.iter();
    args.next(); // Skip the first argument (function name)

    while let Some(i) = args.next() {
        match i.as_str() {
            "-d" | "--digits" => {
                if let Some(arg) = args.next() {
                    digits = arg.parse().unwrap_or_else(|_| {
                        eprintln!("Invalid number of digits: {}", arg);
                        std::process::exit(1);
                    });
                } else {
                    eprintln!("Missing value for digits");
                    std::process::exit(1);
                }
            }
            "-r" | "--reps" => {
                if let Some(arg) = args.next() {
                    reps = arg.parse().unwrap_or_else(|_| {
                        eprintln!("Invalid number of repetitions: {}", arg);
                        std::process::exit(1);
                    });
                } else {
                    eprintln!("Missing value for repetitions");
                    std::process::exit(1);
                }
            }
            "-h" | "--help" => {
                println!("Usage: <number_of_digits> <repetitions>");
                println!("-d, --digits <number>    Set the number of digits (default: 10)");
                println!("-r, --reps <number>      Set the number of repetitions for primality test (default: 20)");
                std::process::exit(0);
            }
            _ => {
                panic!("Unknown argument: {} \n Use -h or --help to see help", i);
            }
        }
    }

    prime_gen(digits, reps)
} 

pub fn prime_gen(digits: u32, reps: u32) -> Integer {
    if digits < 1 {
        panic!("Number of digits must be at least 1");
    }
    // let mut count = 0;
    loop {
        // count += 1;
        // println!("Iteration: {}", count);
        // if count > 1000 {
        //     panic!("Failed to generate a prime number after 1000 attempts");
        // }
        let num = random_num(digits);
        if prime_check(&num, reps) {
            return num;
        }
    }
}

 pub fn prime_check(num: &Integer, reps: u32) -> bool {
    match num.is_probably_prime(reps) {
            IsPrime::No => false,
            _ => true,
        }
}

pub fn random_num(digits: u32) -> Integer {
    let mut big_num = Integer::new();
    let mut rng = rand::rng();
    let first_digit = rng.random_range(1..10);
    let other_digits = (0..digits-1).map(|_| rng.random_range(0..10).to_string()).collect::<String>();
    let random_number = format!("{}{}", first_digit, other_digits);

    big_num.assign(Integer::parse(random_number).unwrap());
    big_num
}
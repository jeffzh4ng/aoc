use std::{error::Error, fs, num::ParseIntError};

#[derive(Debug)]
struct Password<'a> {
    num_one: u32,
    num_two: u32,
    key: char,
    password: &'a str,
}

fn main() -> Result<(), Box<dyn Error>> {
    let path = "./src/bin/2/input.txt";
    let input_str = fs::read_to_string(path)?;

    let passwords = input_str
        .lines()
        .map::<Result<Password, ParseIntError>, _>(|line| {
            let mut split_line = line.split_whitespace();

            let range = split_line.next().unwrap();
            let key = split_line.next().unwrap().chars().nth(0).unwrap();
            let password = split_line.next().unwrap();

            let mut range_split = range.split("-");
            let num_one = range_split.next().unwrap().parse::<u32>()?;
            let num_two = range_split.next().unwrap().parse::<u32>()?;

            Ok(Password {
                num_one,
                num_two,
                key,
                password, // question: how does rust know the str slice we are passing in won't drop during Password's lfietime?
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    println!("{:?}", count_valid_passwords_old_job(&passwords));
    println!("{:?}", (count_valid_passwords_new_job(&passwords)));

    Ok(())
}

fn count_valid_passwords_old_job(passwords: &Vec<Password>) -> u32 {
    let mut count = 0;

    for p in passwords {
        let key_count = p.password.matches(p.key).count() as u32;

        if p.num_one <= key_count && key_count <= p.num_two {
            count += 1
        }
    }

    count
}

fn count_valid_passwords_new_job(passwords: &Vec<Password>) -> u32 {
    let mut count = 0;

    for p in passwords {
        let first_key_matches = p.password.chars().nth((p.num_one - 1) as usize).unwrap() == p.key;
        let second_key_matches = p.password.chars().nth((p.num_two - 1) as usize).unwrap() == p.key;

        if first_key_matches ^ second_key_matches {
            count += 1;
        }
    }

    count
}

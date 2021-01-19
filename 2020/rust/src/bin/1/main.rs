use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufReader, BufRead, Error};

fn main() -> Result<(), Error> {
    let path = "./src/bin/1/input.txt";
    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    let mut nums = Vec::new();
    for line in buffered.lines() {
        let num = line?.parse::<u16>().unwrap();
        nums.push(num);
    }

    println!("{:?}", two_sum_2020(&nums));
    println!("{:?}", three_sum_2020(&nums));

    Ok(())
}

fn two_sum_2020(nums: &Vec<u16>) -> Option<u32> {
    let mut set = HashSet::new();

    for n in nums {
        let factor = 2020 - n;

        if set.contains(&factor) {
            return Some(*n as u32 * factor as u32);
        } else {
            set.insert(n);
        }
    }

    None
}

fn three_sum_2020(nums: &Vec<u16>) -> Option<u32> {
    let mut sorted = nums.clone();
    sorted.sort();

    for a in 0..sorted.len() {
        let target = 2020 - sorted[a];
        let mut l = a+1;
        let mut r = sorted.len() - 1;

        while l < r {
            if sorted[l] + sorted[r] == target {
                return Some(sorted[a] as u32 * sorted[l] as u32 * sorted[r] as u32);
            } else if sorted[l] + sorted[r] < target {
                l += 1;
            } else {
                r -= 1;
            }
        }
    }

    None
}



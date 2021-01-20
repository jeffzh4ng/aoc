use std::{cmp::Ordering, collections::HashSet, error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let path = "./src/bin/1/input.txt";

    let numbers_str = fs::read_to_string(path)?;

    let mut nums = numbers_str
        .lines()
        .map(|num_str| num_str.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;

    println!("{:?}", two_sum_2020(&nums));
    println!("{:?}", three_sum_2020(&mut nums));

    Ok(())
}

fn two_sum_2020(nums: &[u64]) -> Option<u64> {
    let mut set = HashSet::new();

    for n in nums {
        let factor = 2020 - n;

        if set.contains(&factor) {
            return Some(n * factor);
        } else {
            set.insert(n);
        }
    }

    None
}

fn three_sum_2020(nums: &mut Vec<u64>) -> Option<u64> {
    nums.sort();

    for a in 0..nums.len() {
        let target = 2020 - nums[a];
        let mut l = a + 1;
        let mut r = nums.len() - 1;

        while l < r {
            match (nums[l] + nums[r]).cmp(&target) {
                Ordering::Equal => return Some(nums[a] * nums[l] * nums[r]),
                Ordering::Less => l += 1,
                Ordering::Greater => r -= 1,
            }
        }
    }

    None
}

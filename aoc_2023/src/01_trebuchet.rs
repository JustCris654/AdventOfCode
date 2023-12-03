use std::{char, fs::read_to_string};

fn main() {
    let file_path = "input.txt";

    let sum: u32 = read_to_string(file_path)
        .expect("Cannot read file")
        .lines()
        .map(String::from)
        .map(|line| line.chars().filter(|&c| c >= '0' && c <= '9').collect())
        .map(|nums: Vec<char>| match nums.len() {
            0 => 0,
            1 => nums.get(0).unwrap().to_digit(10).unwrap() * 11,
            _ => String::from_iter([nums[0], nums[nums.len() - 1]])
                .parse::<u32>()
                .unwrap(),
        })
        .sum();

    println!("{}", sum);
}

use std::{char, fs::read_to_string};

fn main() {
    let file_path = "input.txt";

    let mut sum = 0;

    let content: Vec<String> = read_to_string(file_path)
        .expect("Cannot read file")
        .lines()
        .map(String::from)
        .collect();

    for line in content {
        let nums: Vec<char> = line.chars().filter(|&c| c >= '0' && c <= '9').collect();
        if nums.len() == 1 {
            let num = nums.get(0).unwrap().to_digit(10).unwrap() * 11;
            sum += num;
        } else if nums.len() > 1 {
            let num = String::from_iter([nums[0], nums[nums.len() - 1]])
                .parse::<u32>()
                .unwrap();
            sum += num;
        }
    }

    println!("{}", sum);
}

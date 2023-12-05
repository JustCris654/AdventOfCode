use std::{char, fs::read_to_string};

fn main() {
    let file_path = "input.txt";
    let content = read_to_string(file_path).expect("Cannot read file");

    day_01(&content);
    day_02(&content);
}

fn day_01(content: &String) {
    let sum: u32 = content
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

#[derive(Debug)]
struct NumMatch {
    index: usize,
    num: u32,
}

fn find_nums_inline(nums_words: &Vec<&str>, line: &String) -> (u32, u32) {
    let mut nums: Vec<NumMatch> = vec![];
    for (i, num_word) in nums_words.iter().enumerate() {
        if let Some(idx) = line.find(num_word) {
            nums.push(NumMatch {
                index: idx,
                num: (i / 2 + 1) as u32,
            });
        }

        if let Some(idx) = line.rfind(num_word) {
            nums.push(NumMatch {
                index: idx,
                num: (i / 2 + 1) as u32,
            });
        }
    }

    nums.sort_by(|a, b| a.index.cmp(&b.index));

    (nums.first().unwrap().num, nums.last().unwrap().num)
}

fn day_02(content: &String) {
    let nums_words = vec![
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];

    let mut sum: u32 = 0;

    let lines: Vec<String> = content.lines().map(String::from).collect();

    for line in lines {
        let nums = find_nums_inline(&nums_words, &line);
        let num = nums.0 * 10 + nums.1;
        sum += num;
    }

    println!("{}", sum);
}

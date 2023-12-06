use std::{collections::HashSet, fs::read_to_string};

const MAX_REDS: u32 = 12;
const MAX_GREENS: u32 = 13;
const MAX_BLUES: u32 = 14;

fn main() {
    let file_path = "input.txt";
    let content = read_to_string(file_path).expect("Cannot read file");

    day_01(&content);
    day_02(&content);
}

fn day_01(content: &String) {
    let mut impossible_games: HashSet<usize> = HashSet::new();

    for (i, line) in content.lines().enumerate() {
        let mut splitted = line.split(" ");

        let (mut reds, mut greens, mut blues): (u32, u32, u32) = (0, 0, 0);
        while let (Some(value), Some(identifier)) = (splitted.next(), splitted.next()) {
            if let Some(_) = identifier.find("green") {
                greens += value.parse::<u32>().unwrap();
            }
            if let Some(_) = identifier.find("red") {
                reds += value.parse::<u32>().unwrap();
            }
            if let Some(_) = identifier.find("blue") {
                blues += value.parse::<u32>().unwrap();
            }

            if identifier.chars().nth(identifier.len() - 1).unwrap() == ';'
                || splitted.clone().peekable().peek().is_none()
            {
                if greens > MAX_GREENS || reds > MAX_REDS || blues > MAX_BLUES {
                    impossible_games.insert(i + 1);
                    break;
                }

                reds = 0;
                greens = 0;
                blues = 0;
            }
        }
    }

    let lines_num = content.lines().count();
    let res: usize = lines_num * (lines_num + 1) / 2 - impossible_games.iter().sum::<usize>();

    println!("{}", res);
}

fn day_02(content: &String) {
    let mut powers: Vec<u32> = vec![];

    for line in content.lines() {
        let mut splitted = line.split(" ");

        let (mut reds, mut greens, mut blues): (Vec<u32>, Vec<u32>, Vec<u32>) =
            (vec![], vec![], vec![]);
        while let (Some(value), Some(identifier)) = (splitted.next(), splitted.next()) {
            if let Some(_) = identifier.find("green") {
                greens.push(value.parse::<u32>().unwrap());
            }
            if let Some(_) = identifier.find("red") {
                reds.push(value.parse::<u32>().unwrap());
            }
            if let Some(_) = identifier.find("blue") {
                blues.push(value.parse::<u32>().unwrap());
            }

            if splitted.clone().peekable().peek().is_none() {
                let reds = reds.iter().max().unwrap();
                let greens = greens.iter().max().unwrap();
                let blues = blues.iter().max().unwrap();

                powers.push(reds * greens * blues);
            }
        }
    }

    let res: u32 = powers.iter().sum();

    println!("{}", res);
}

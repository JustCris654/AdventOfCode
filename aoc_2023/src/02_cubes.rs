use std::{collections::HashSet, fs::read_to_string};

const MAX_REDS: u32 = 12;
const MAX_GREENS: u32 = 13;
const MAX_BLUES: u32 = 14;

fn main() {
    let file_path = "input.txt";
    let content = read_to_string(file_path).expect("Cannot read file");

    day_01(&content);
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

use std::{fs::read_to_string, str::Lines};

fn main() {
    let file_path = "input.txt";
    let content = read_to_string(file_path).expect("Cannot read file");

    day_01(&content);
    // day_02(&content);
}

fn check_adiacents<'a>(lines: &mut Lines<'a>, i: &usize, j: &usize) -> bool {
    for a in 0..3 {
        for b in 0..3 {
            let idx = *i as i32 + a as i32 - 1;

            if idx >= 0 {
                if let Some(line) = lines.clone().nth(idx as usize) {
                    let mut line = line.chars();
                    let idx = *j as i32 + b as i32 - 1;
                    if idx >= 0 {
                        match line.nth(idx as usize) {
                            Some(c) => {
                                if !c.is_digit(10) && c != '.' {
                                    return true;
                                }
                            }
                            None => (),
                        }
                    }
                }
            }
        }
    }

    false
}

fn calc_num(num: &Vec<u32>) -> u32 {
    let base: u32 = 10;
    num.iter()
        .rev()
        .enumerate()
        .map(|(i, n)| base.pow(i as u32) * n)
        .sum()
}

fn day_01(content: &String) {
    let mut lines = content.lines();
    // let line_len = lines.nth(0).unwrap().len();

    let mut sum: u32 = 0;
    let mut num: Vec<u32> = vec![];
    let mut to_sum = false;

    content.lines().enumerate().for_each(|(i, line)| {
        line.chars().enumerate().for_each(|(j, c)| {
            if c.is_digit(10) {
                num.push(c.to_digit(10).unwrap());

                to_sum = check_adiacents(&mut lines, &i, &j) || to_sum;
            } else {
                // check for symbols around the num

                if to_sum {
                    sum += calc_num(&num);
                }

                num.clear();
                to_sum = false;
            }
        });
        if to_sum {
            sum += calc_num(&num);
        }

        num.clear();
        to_sum = false;
    });

    println!("Sum: {}", sum);
}


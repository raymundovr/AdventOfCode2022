use anyhow::Result;
use regex::Regex;
use std::fs::read_to_string;
use std::collections::VecDeque;

fn get_initial_stacks(file_contents: &str, stacks: usize) -> Vec<VecDeque<String>> {
    let mut crates = Vec::new();

    for _i in 0..stacks {
        crates.push(VecDeque::new());
    }

    for line in file_contents.split('\n') {
        if line.is_empty() {
            break;
        }
        let mut i = 0;
        let mut stack = 0;
        while i < line.len() {
            let slice = &line[i..i + 2];
            if slice.starts_with('[') {
                crates[stack].push_back(slice.chars().nth(1).unwrap().to_string());
            }

            i += 4;
            stack += 1;
        }
    }

    crates
}

fn main() -> Result<()> {
    let contents = read_to_string("src/inputs/inputday5")?;
    let first_line = contents.lines().next().expect("Cannot read first line");

    let stacks = (first_line.len() + 1) / 4;
    let mut crates = get_initial_stacks(&contents, stacks);

    let regex = Regex::new(r"move (\d{1,2}) from (\d) to (\d)").unwrap();

    for line in contents.lines() {        
        if let Some(cap) = regex.captures(line) {
            let (elements, s0, s1) = (
                cap[1].parse::<usize>().unwrap(),
                cap[2].parse::<usize>().unwrap(),
                cap[3].parse::<usize>().unwrap(),
            );

            for i in 0..elements {
                if let Some(e) = crates[s0-1].pop_front() {
                    crates[s1-1].insert(i, e);
                } else {
                    eprintln!("Something wrong with line {}", line);
                }
            }
        }
    }
    println!("Answer:");
    for deque in crates {
        print!("{:?}", deque.front().unwrap());
    }
    Ok(())
}

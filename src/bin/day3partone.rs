use std::fs::read_to_string;
use anyhow::Result;

fn main() -> Result<()> {
    println!("Solution to day 3");
    let contents = read_to_string("inputs/inputday3")?;
    let mut result = 0;

    for line in contents.split('\n').filter(|l| !l.is_empty()) {
        let (left, right) = line.split_at(line.len() / 2);
        for ch in left.chars() {
            if right.chars().any(|c| c == ch) {
                let value = if ch.is_ascii_lowercase() { ch as u32 - 96 } else { ch as u32 - 38 };
                println!("Value {} {}", ch, value);
                result += value;
                break;
            }
        }
    }

    println!("Result {}", result);
    Ok(())
}
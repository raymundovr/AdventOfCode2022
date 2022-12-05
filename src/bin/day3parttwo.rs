use anyhow::Result;
use std::fs::read_to_string;

fn main() -> Result<()> {
    println!("Solution to day 3");
    let contents = read_to_string("src/inputs/inputday3")?;
    let mut result = 0;
    let mut group_item = 1;
    let mut group: Vec<&str> = Vec::new();

    for line in contents.split("\n").filter(|l| l.len() > 0) {
        if group_item < 3 {
            group.push(line);
            group_item += 1;
        } else {
            let [one, two] = [group[0], group[1]];
            for ch in one.chars() {
                if two.chars().find(|c| c == &ch).is_some()
                    && line.chars().find(|c| c == &ch).is_some()
                {
                    let value = if ch.is_ascii_lowercase() {
                        ch as u32 - 96
                    } else {
                        ch as u32 - 38
                    };
                    println!("Value {} {}", ch, value);
                    result += value;
                    break;
                }
            }

            group_item = 1;
            group = Vec::new();
        }
    }

    println!("Result {}", result);
    Ok(())
}

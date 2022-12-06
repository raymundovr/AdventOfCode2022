use anyhow::Result;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let input = read_to_string("src/inputs/inputday4")?;

    let result = input.lines().filter(|l| l.len() > 1).fold(0, |acc, line| {
        if let Some((left, right)) = line.split_once(',') {
            let (start_left, end_left) = left.split_once('-').unwrap();
            let (start_right, end_right) = right.split_once('-').unwrap();
            let (start_left, end_left) = (
                start_left.parse::<usize>().unwrap(),
                end_left.parse::<usize>().unwrap(),
            );

            let (start_right, end_right) = (
                start_right.parse::<usize>().unwrap(),
                end_right.parse::<usize>().unwrap(),
            );
            let mut incr = acc;
            if (start_left <= start_right && end_left >= end_right)
                || (start_right <= start_left && end_right >= end_left)
            {
                incr += 1;
            }

            return incr;
        }
        acc
    });

    println!("Result is {}", result);

    Ok(())
}

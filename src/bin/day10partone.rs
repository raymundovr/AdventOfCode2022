use anyhow::Result;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let input = read_to_string("inputs/inputday10")?;
    let mut x = 1;
    let mut cycles: i32 = 0;
    let mut consideration: i32 = 20;
    let mut strength = 0;

    for line in input.lines() {
        if line == "noop" {
            cycles += 1;
        } else {
            let (_instruction, argument) = line.split_once(' ').unwrap();
            cycles += 1;
            if cycles == consideration {
                strength += consideration * x;
                consideration += 40;
            }

            cycles += 1;
            if cycles == consideration {
                strength += consideration * x;
                consideration += 40;
            }
            x += argument.parse::<i32>().unwrap();
        }

        if cycles == consideration {
            strength += consideration * x;
            consideration += 40;
        }
    }

    println!("Strength: {}", strength);

    Ok(())
}
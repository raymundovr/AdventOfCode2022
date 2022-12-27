use anyhow::Result;
use std::fs::read_to_string;

fn paint_beam(cycles: i32, x: i32) {
    if cycles % 40 == 0 {
        println!();
    }
    if i32::abs((cycles % 40) - x) <= 1 {
        print!("😉");
    } else {
        print!("⚫️");
    }
}

fn main() -> Result<()> {
    let input = read_to_string("inputs/inputday10")?;
    let mut x = 1;
    let mut cycles: i32 = 0;

    for line in input.lines() {
        paint_beam(cycles, x);
        if line == "noop" {
            cycles += 1;
        } else {
            let (_instruction, argument) = line.split_once(' ').unwrap();
            cycles += 1;
            paint_beam(cycles, x);
            cycles += 1;
            x += argument.parse::<i32>().unwrap();
        }
    }

    Ok(())
}

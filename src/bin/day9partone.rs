use anyhow::Result;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() -> Result<()> {
    let input = read_to_string("inputs/inputday9")?;
    let mut h_current_position = (0, 0);
    let mut h_previous_position = (0, 0);
    let mut t_current_position = (0, 0);
    let mut t_positions: HashSet<(i32, i32)> = HashSet::new();
    let is_t_still_ok = |a: &(i32, i32), b: &(i32, i32)| { i32::abs(a.0 - b.0) <= 1 && i32::abs(a.1 - b.1) <= 1 };

    for line in input.lines() {
        println!("Interpreting {}", line);
        let (direction, positions) = line.split_once(' ').unwrap();

        for _i in 1..=positions.parse::<i32>().unwrap() {
            h_previous_position = h_current_position;
            match direction {
                "L" => {
                    h_current_position = (h_current_position.0 - 1, h_current_position.1);
                }
                "R" => {
                    h_current_position = (h_current_position.0 + 1, h_current_position.1);
                }
                "U" => {
                    h_current_position = (h_current_position.0, h_current_position.1 + 1);
                }
                "D" => {
                    h_current_position = (h_current_position.0, h_current_position.1 - 1);
                }
                _ => {
                    panic!("Unimplemented!");
                }
            }

            if !is_t_still_ok(&h_current_position, &t_current_position) {
                t_current_position = h_previous_position;
            }
            t_positions.insert(t_current_position);
        }
    }

    println!("T visited positions {}", t_positions.len());

    Ok(())
}

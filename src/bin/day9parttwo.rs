use anyhow::Result;
use std::{collections::HashSet, fs::read_to_string};

fn get_next_position(previous_knot: &(i32, i32), my_position: &(i32, i32)) -> (i32, i32) {
    let dif_x = previous_knot.0 - my_position.0;
    let dif_y = previous_knot.1 - my_position.1;

    if i32::abs(dif_x) <= 1 && i32::abs(dif_y) <= 1 {
        return (my_position.0, my_position.1);
    } else if dif_x > 0 && dif_y == 0 {
        return (my_position.0 + 1, my_position.1);
    } else if dif_x == 0 && dif_y > 0 {
        return (my_position.0, my_position.1 + 1);
    } else if dif_x < 0 && dif_y == 0 {
        return (my_position.0 - 1, my_position.1);
    } else if dif_x == 0 && dif_y < 0 {
        return (my_position.0, my_position.1 - 1);
    } else if dif_x > 0 && dif_y > 0 {
        return (my_position.0 + 1, my_position.1 + 1);
    } else if dif_x > 0 && dif_y < 0 {
        return (my_position.0 + 1, my_position.1 - 1);
    } else if dif_x < 0 && dif_y < 0 {
        return (my_position.0 - 1, my_position.1 - 1);
    } else {
        return (my_position.0 - 1, my_position.1 + 1);
    }
}

fn main() -> Result<()> {
    let input = read_to_string("inputs/inputday9")?;
    let mut t_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut h = (0, 0);
    let mut rope: [(i32, i32); 9] = [(0, 0); 9];

    for line in input.lines() {
        let (direction, steps) = line.split_once(' ').unwrap();
        for _i in 0..steps.parse::<i32>().unwrap() {
            match direction {
                "U" => {
                    h = (h.0 + 1, h.1);
                }
                "D" => {
                    h = (h.0 - 1, h.1);
                }
                "L" => {
                    h = (h.0, h.1 - 1);
                }
                "R" => {
                    h = (h.0, h.1 + 1);
                }
                _ => {
                    unimplemented!("Invalid direction");
                }
            }

            rope[0] = get_next_position(&h, &rope[0]);
            for k in 1..rope.len() {
                rope[k] = get_next_position(&rope[k - 1], &rope[k]);
            }

            t_positions.insert(rope[8]);
        }
    }

    println!("Head {:?}", h);
    println!("Rope {:?}", rope);
    println!("Stored {:?}", t_positions);
    println!("Tail positions {}", t_positions.len());
    Ok(())
}

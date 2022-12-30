use anyhow::Result;
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

type Point = (usize, usize);
type Map = Vec<Vec<u32>>;
type Positions = Vec<Point>;
type Visited = HashMap<Point, usize>;

fn get_next_from(location: Point, map: &Map) -> Positions {
    let mut next = Vec::new();
    let current = map[location.0][location.1];

    // Up
    if location.0 > 0 && map[location.0 - 1][location.1] <= current + 1 {
        next.push((location.0 - 1, location.1));
    }

    // Down
    if location.0 < map.len() - 1 && map[location.0 + 1][location.1] <= current + 1 {
        next.push((location.0 + 1, location.1));
    }

    // Left
    if location.1 > 0 && map[location.0][location.1 - 1] <= current + 1 {
        next.push((location.0, location.1 - 1));
    }

    // Right
    if location.1 < map[location.0].len() - 1 && map[location.0][location.1 + 1] <= current + 1 {
        next.push((location.0, location.1 + 1));
    }

    next
}

fn find_path_q(map: &Map, start: Point, end: Point) {
    let mut q: VecDeque<(usize, usize)> = VecDeque::from([start]);
    let mut visited: Visited = HashMap::new();
    visited.insert(start, 0);

    while !q.is_empty() {
        let current = q.pop_front().unwrap();
        let current_cost = *visited.get(&current).unwrap();

        for position in get_next_from(current, map) {
            if position == end {
                println!("Destination reached {}", current_cost + 1);
                break;
            }

            if !q.contains(&position) {
                let new_cost = current_cost + 1;
                match visited.get(&position) {
                    Some(cost) => {
                        if *cost > new_cost {
                            *visited.get_mut(&position).unwrap() = new_cost;
                            q.push_back(position);
                        }
                    },
                    None => {
                        visited.insert(position, new_cost);
                        q.push_back(position);
                    }                    
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let contents = read_to_string("inputs/inputday12")?;
    let mut map: Map = Vec::new();
    let mut x = 0;
    let mut start: Point = (0, 0);
    let mut end: Point = (0, 0);

    for line in contents.lines() {
        let mut line_vec = Vec::new();
        let mut y = 0;
        for c in line.chars() {
            let value = match c.is_ascii_lowercase() {
                true => c as u32 - 96,
                false => {
                    if c == 'S' {
                        start = (x, y);
                        0
                    } else {
                        end = (x, y);
                        27
                    }
                }
            };
            y += 1;
            line_vec.push(value);
        }
        x += 1;
        map.push(line_vec);
    }

    find_path_q(&map, start, end);

    Ok(())
}

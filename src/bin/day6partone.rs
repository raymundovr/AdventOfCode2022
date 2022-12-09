use anyhow::Result;
use std::fs::read_to_string;

fn has_duplicates(slice: &str) -> bool {
    for (i, c) in slice.char_indices() {
        match slice.rfind(c) {
            Some(pos) => {
                if pos != i {
                    return true;
                }
            },
            _ => {}
        }
    }

    false
}

fn main() -> Result<()> {
    let contents = read_to_string("inputs/inputday6")?;
    let size: usize = std::env::args().nth(1).expect("Specify 4 or 14 characters slice size").parse().expect("That's not a number");

    for i in 0..contents.len() - 4 {
        let slice = &contents[i..i+size];
        if !has_duplicates(slice) {
            println!("Found {}. Answer: {}", slice, size + i);
            break;
        }
    }

    Ok(())
}
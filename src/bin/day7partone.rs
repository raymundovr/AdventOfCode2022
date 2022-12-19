use adventofcode::Directory;
use anyhow::Result;
use std::{cell::RefCell, fs::read_to_string};

fn main() -> Result<()> {
    let contents = read_to_string("inputs/inputday7")?;
    let mut tree: Vec<RefCell<Directory>> = Vec::new();
    tree.push(RefCell::new(Directory {
        index: 1,
        name: "/".to_string(),
        size: 0,
        parent: 0,
    }));
    let mut current_node_index: usize = 0;
    let mut is_listing = false;

    for line in contents.lines() {
        if line.starts_with("$") {
            is_listing = false;
            let mut args = line.split(' ');
            args.next();
            let command = args.nth(0).expect("Cannot get command");
            if command == "cd" {
                let dir = args.nth(0).expect("Not enough arguments for cd");
                if dir == ".." {
                    let node = tree
                        .iter()
                        .find(|rn| rn.borrow().index == current_node_index)
                        .expect(&format!("Cannot find node {}", current_node_index));
                    let parent = tree
                        .iter()
                        .find(|rn| rn.borrow().index == node.borrow().parent)
                        .expect(&format!("Cannot find parent node {}", current_node_index));
                    current_node_index = parent.borrow().index;
                } else {
                    let node = tree
                        .iter()
                        .find(|rn| {
                            rn.borrow().name == dir.to_string()
                                && rn.borrow().parent == current_node_index
                        })
                        .expect(&format!(
                            "Cannot find node {} - {}",
                            current_node_index, dir
                        ));
                    current_node_index = node.borrow().index;
                }
            } else if command == "ls" {
                is_listing = true;
            }
        } else if is_listing {
            let (arg_0, arg_1) = line.split_once(" ").unwrap();
            if arg_0 == "dir" {
                let index = tree.len() + 1;
                let node = Directory {
                    index,
                    name: arg_1.to_string(),
                    size: 0,
                    parent: current_node_index,
                };
                tree.push(RefCell::new(node));
            } else {
                let file_size = arg_0.parse::<usize>().expect("Cannot parse file size");
                let mut current_node_result = tree
                    .iter()
                    .find(|n: &&RefCell<Directory>| n.borrow().index == current_node_index);
                while let Some(current_node) = current_node_result {
                    current_node.borrow_mut().update_size(file_size);
                    current_node_result = tree.iter().find(|n: &&RefCell<Directory>| {
                        n.borrow().index == current_node.borrow().parent
                    });
                }
            }
        }
    }

    
    let result = tree.iter().fold(0, |mut acc, d| {
        if d.borrow().size < 100000 {
            println!("Dir {} - Size {}", d.borrow().name, d.borrow().size);
            acc += d.borrow().size;
        }
        acc
    });

    println!("Result: {:?}", result);
    Ok(())
}

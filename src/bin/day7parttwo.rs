use adventofcode::Directory;
use anyhow::Result;
use std::{fs::read_to_string, cell::RefCell};

const TOTAL_DISK_SPACE: usize = 70_000_000;
const NEEDED_SPACE: usize = 30_000_000;

fn main() -> Result<()> {
    let contents = read_to_string("inputs/inputday7")?;
    let mut tree: Vec<RefCell<Directory>> = Vec::new();
    tree.push(RefCell::new(Directory { index: 1, name: "/".to_string(), size: 0, parent: 0 }));
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
                    let node = tree.iter().find(|rn| rn.borrow().index == current_node_index).expect(&format!("Cannot find node {}", current_node_index));
                    let parent = tree.iter().find(|rn| rn.borrow().index == node.borrow().parent).expect(&format!("Cannot find parent node {}", current_node_index));
                    current_node_index = parent.borrow().index;
                } else {
                    let node = tree.iter().find(|rn| rn.borrow().name == dir.to_string() && rn.borrow().parent == current_node_index).expect(&format!("Cannot find node {} - {}", current_node_index, dir));
                    current_node_index = node.borrow().index;
                }
            } else if command == "ls" {
                is_listing = true;
            }
        } else if is_listing {
            let (arg_0, arg_1) = line.split_once(" ").unwrap();
            if arg_0 == "dir" {
                let index = tree.len() + 1;
                let node = Directory { index, name: arg_1.to_string(), size: 0, parent: current_node_index };
                tree.push(RefCell::new(node));
            } else {
                let file_size = arg_0.parse::<usize>().expect("Cannot parse file size");
                let mut current_node_result = tree.iter().find(|n: &&RefCell<Directory>| { n.borrow().index == current_node_index });
                while let Some(current_node) = current_node_result {
                    current_node.borrow_mut().update_size(file_size);
                    current_node_result = tree.iter().find(|n: &&RefCell<Directory>| { n.borrow().index == current_node.borrow().parent });
                }
            }
        }
    }

    let root = tree.iter().find(|n| { n.borrow().index == 1 }).unwrap();
    let current_unused_space = TOTAL_DISK_SPACE - root.borrow().size;
    let need_space = NEEDED_SPACE - current_unused_space;
    println!("Current unused space {}. Need {}", current_unused_space, need_space);
    let result = tree.iter().filter(|n| { n.borrow().size >= need_space  }).min();

    println!("Result {:?}", result);
    Ok(())
}
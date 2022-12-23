use anyhow::Result;
use std::collections::HashSet;
use std::fs::read_to_string;

fn get_scenic_score(position: (usize, usize), matrix: &Vec<Vec<usize>>) -> usize {
    let (i, j) = position;
    let max = matrix[i][j];
    let mut scene = (0, 0, 0, 0);

    // arriba
    for posi in (0..i).rev() {
        scene.0 += 1;
        if matrix[posi][j] >= max {
            break;
        }
    }

    // abajo
    for posi in (i + 1)..matrix.len() {
        scene.1 += 1;

        if matrix[posi][j] >= max {
            break;
        }
    }

    // derecha
    for posj in (j + 1)..matrix[0].len() {
        scene.2 += 1;
        
        if matrix[i][posj] >= max {
            break;
        }
    }

    // izq
    for posj in (0..j).rev() {
        scene.3 += 1;
        if matrix[i][posj] >= max {
            break;
        }

    }

    scene.0 * scene.1 * scene.2 * scene.3
}

fn main() -> Result<()> {
    let contents = read_to_string("inputs/inputday8")?;

    let matrix: Vec<Vec<usize>> = contents
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect()
        })
        .collect();

    let edges_count = 2 * matrix.len() + 2 * matrix[0].len() - 4;
    let mut visibles: HashSet<(usize, usize)> = HashSet::new();

    // from the top
    let row = &matrix[0];
    for j in 1..row.len() - 1 {
        let mut max = matrix[0][j];
        for i in 1..matrix.len() - 1 {
            if matrix[i][j] > max {
                max = matrix[i][j];
                visibles.insert((i, j));
            }
        }
    }

    // from the bottom
    let row = &matrix[matrix.len() - 1];
    for j in 1..row.len() - 1 {
        let mut max = matrix[matrix.len() - 1][j];
        for i in (1..matrix.len() - 1).rev() {
            if matrix[i][j] > max {
                max = matrix[i][j];
                visibles.insert((i, j));
            }
        }
    }

    // from the left
    for i in 1..matrix.len() - 1 {
        let mut max = matrix[i][0];
        for j in 1..matrix.len() - 1 {
            if matrix[i][j] > max {
                max = matrix[i][j];
                visibles.insert((i, j));
            }
        }
    }

    // from the right
    for i in (1..matrix.len() - 1).rev() {
        let mut max = matrix[i][matrix.len() - 1];
        for j in (1..matrix.len() - 1).rev() {
            if matrix[i][j] > max {
                max = matrix[i][j];
                visibles.insert((i, j));
            }
        }
    }
    println!("Visible elements: {}", edges_count + visibles.len());

    let mut max_scenic = 0;
    for element in visibles {
        let score = get_scenic_score(element, &matrix);
        if score > max_scenic {
            max_scenic = score;
            println!("Element {:?} => {}, score: {}", element, matrix[element.0][element.1], score);
        }
    }

    println!("Max scenic score {}", max_scenic);
    Ok(())
}

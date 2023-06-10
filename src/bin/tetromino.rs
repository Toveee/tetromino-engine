use colored::*;
use rand::seq::SliceRandom;
use std::fmt;

#[derive(Clone)]
struct Tetromino {
    blocks: Vec<Vec<Option<Block>>>,
    shape: Shape,
}

#[derive(Clone)]
struct Block {
    color: Color,
}

#[derive(Clone)]
enum Shape {
    I,
    J,
    L,
    O,
    S,
    Z,
    T,
}

impl fmt::Display for Tetromino {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.blocks {
            for cell in row {
                let ch = match cell {
                    Some(_) => "X".color(cell.clone().unwrap().color).bold(),
                    None => " ".normal(),
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn init_i() -> Tetromino {
    let block = Some(Block { color: Color::Cyan });

    Tetromino {
        blocks: vec![vec![
            block.clone(),
            block.clone(),
            block.clone(),
            block.clone(),
        ]],
        shape: Shape::I,
    }
}

fn init_j() -> Tetromino {
    let block = Some(Block { color: Color::Blue });

    Tetromino {
        blocks: vec![
            vec![block.clone(), None, None],
            vec![block.clone(), block.clone(), block.clone()],
        ],
        shape: Shape::J,
    }
}

fn init_l() -> Tetromino {
    let block = Some(Block {
        color: Color::TrueColor {
            r: 255,
            g: 140,
            b: 0,
        },
    });

    Tetromino {
        blocks: vec![
            vec![None, None, block.clone()],
            vec![block.clone(), block.clone(), block.clone()],
        ],
        shape: Shape::L,
    }
}

fn init_o() -> Tetromino {
    let block = Some(Block {
        color: Color::Yellow,
    });

    Tetromino {
        blocks: vec![
            vec![block.clone(), block.clone()],
            vec![block.clone(), block.clone()],
        ],
        shape: Shape::O,
    }
}

fn init_s() -> Tetromino {
    let block = Some(Block {
        color: Color::Green,
    });

    Tetromino {
        blocks: vec![
            vec![None, block.clone(), block.clone()],
            vec![block.clone(), block.clone(), None],
        ],
        shape: Shape::S,
    }
}

fn init_z() -> Tetromino {
    let block = Some(Block { color: Color::Red });

    Tetromino {
        blocks: vec![
            vec![block.clone(), block.clone(), None],
            vec![None, block.clone(), block.clone()],
        ],
        shape: Shape::Z,
    }
}

fn init_t() -> Tetromino {
    let block = Some(Block {
        color: Color::TrueColor {
            r: 128,
            g: 0,
            b: 128,
        },
    });

    Tetromino {
        blocks: vec![
            vec![None, block.clone(), None],
            vec![block.clone(), block.clone(), block.clone()],
        ],
        shape: Shape::T,
    }
}

fn generate_bag() -> Vec<Tetromino> {
    let mut tetrominoes: Vec<Tetromino> = Vec::new();
    tetrominoes.push(init_i());
    tetrominoes.push(init_j());
    tetrominoes.push(init_l());
    tetrominoes.push(init_o());
    tetrominoes.push(init_s());
    tetrominoes.push(init_z());
    tetrominoes.push(init_t());
    tetrominoes.shuffle(&mut rand::thread_rng());
    tetrominoes
}

fn matrix_transpose<T: Clone>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let mut transposed = vec![Vec::with_capacity(matrix.len()); matrix[0].len()];
    for row in matrix {
        for (i, val) in row.iter().enumerate() {
            transposed[i].push(val.clone());
        }
    }
    transposed
}

fn matrix_reflect<T: Clone>(m: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let reflected: Vec<Vec<T>> = m
        .iter()
        .map(|row| row.iter().cloned().rev().collect())
        .collect();
    reflected
}

impl Tetromino {
    fn rotate_cw(&mut self) {
        self.blocks = matrix_transpose(&self.blocks);
        self.blocks = matrix_reflect(&self.blocks)
    }

    fn rotate_ccw(&mut self) {
        self.blocks = matrix_reflect(&self.blocks);
        self.blocks = matrix_transpose(&self.blocks);
    }
}

fn main() {
    let tetrominoes: Vec<Tetromino> = generate_bag();

    for tetromino in tetrominoes.iter() {
        let mut rotated: Tetromino = tetromino.clone();
        rotated.rotate_cw();
        let mut restored: Tetromino = rotated.clone();
        restored.rotate_ccw();
        println!(
            "Before:\n{}\nAfter:\n{}\nRestored: \n{}",
            tetromino,
            rotated,
            restored // "Before:\n{}\nAfter:\n{}\n",
                     // tetromino, rotated
        );
    }
}

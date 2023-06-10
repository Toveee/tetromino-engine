use colored::*;
use rand::seq::SliceRandom;
use std::fmt;

#[derive(Clone)]
pub struct Tetromino {
    blocks: Vec<Vec<Option<Block>>>,
    // TODO: Remove or utilize this
    _shape: Shape,
}

#[derive(Clone)]
pub enum Shape {
    I,
    J,
    L,
    O,
    S,
    Z,
    T,
}

#[derive(Clone)]
struct Block {
    color: Color,
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
        _shape: Shape::I,
    }
}

fn init_j() -> Tetromino {
    let block = Some(Block { color: Color::Blue });

    Tetromino {
        blocks: vec![
            vec![block.clone(), None, None],
            vec![block.clone(), block.clone(), block.clone()],
        ],
        _shape: Shape::J,
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
        _shape: Shape::L,
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
        _shape: Shape::O,
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
        _shape: Shape::S,
    }
}

fn init_z() -> Tetromino {
    let block = Some(Block { color: Color::Red });

    Tetromino {
        blocks: vec![
            vec![block.clone(), block.clone(), None],
            vec![None, block.clone(), block.clone()],
        ],
        _shape: Shape::Z,
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
        _shape: Shape::T,
    }
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

fn generate_bag() -> Vec<Tetromino> {
    let mut tetrominoes: Vec<Tetromino> = Vec::new();
    tetrominoes.push(Tetromino::new(Shape::I));
    tetrominoes.push(Tetromino::new(Shape::J));
    tetrominoes.push(Tetromino::new(Shape::L));
    tetrominoes.push(Tetromino::new(Shape::O));
    tetrominoes.push(Tetromino::new(Shape::S));
    tetrominoes.push(Tetromino::new(Shape::Z));
    tetrominoes.push(Tetromino::new(Shape::T));
    tetrominoes.shuffle(&mut rand::thread_rng());
    tetrominoes
}

impl Tetromino {
    pub fn new(shape: Shape) -> Tetromino {
        match shape {
            Shape::I => init_i(),
            Shape::J => init_j(),
            Shape::L => init_l(),
            Shape::O => init_o(),
            Shape::S => init_s(),
            Shape::Z => init_z(),
            Shape::T => init_t(),
        }
    }

    pub fn rotate_cw(&mut self) {
        self.blocks = matrix_transpose(&self.blocks);
        self.blocks = matrix_reflect(&self.blocks)
    }

    pub fn rotate_ccw(&mut self) {
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
            restored
        );
    }
}

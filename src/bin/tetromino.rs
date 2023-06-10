use colored::*;
use rand::seq::SliceRandom;
use std::fmt;

#[derive(Clone)]
struct Tetromino {
    blocks: Vec<Vec<u8>>,
    shape: Shape,
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
        let color = self.color();

        for row in &self.blocks {
            for cell in row {
                let ch = if *cell == 1 {
                    "X".color(color).bold()
                } else {
                    " ".normal()
                };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn init_i() -> Tetromino {
    Tetromino {
        blocks: vec![vec![1, 1, 1, 1]],
        shape: Shape::I,
    }
}

fn init_j() -> Tetromino {
    Tetromino {
        blocks: vec![vec![1, 0, 0], vec![1, 1, 1]],
        shape: Shape::J,
    }
}

fn init_l() -> Tetromino {
    Tetromino {
        blocks: vec![vec![0, 0, 1], vec![1, 1, 1]],
        shape: Shape::L,
    }
}

fn init_o() -> Tetromino {
    Tetromino {
        blocks: vec![vec![1, 1], vec![1, 1]],
        shape: Shape::O,
    }
}

fn init_s() -> Tetromino {
    Tetromino {
        blocks: vec![vec![0, 1, 1], vec![1, 1, 0]],
        shape: Shape::S,
    }
}

fn init_z() -> Tetromino {
    Tetromino {
        blocks: vec![vec![1, 1, 0], vec![0, 1, 1]],
        shape: Shape::Z,
    }
}

fn init_t() -> Tetromino {
    Tetromino {
        blocks: vec![vec![0, 1, 0], vec![1, 1, 1]],
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

    fn color(&self) -> colored::Color {
        match self.shape {
            Shape::I => Color::Cyan,
            Shape::J => Color::Blue,
            Shape::L => colored::Color::TrueColor {
                r: 255,
                g: 140,
                b: 0,
            },
            Shape::O => Color::Yellow,
            Shape::S => Color::Green,
            Shape::Z => Color::Red,
            Shape::T => colored::Color::TrueColor {
                r: 128,
                g: 0,
                b: 128,
            },
        }
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
            tetromino, rotated, restored
            // "Before:\n{}\nAfter:\n{}\n",
            // tetromino, rotated
        );
    }
}

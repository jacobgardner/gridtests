use std::{fmt::Display, thread::panicking};

use super::grid::Grid;

#[derive(Clone)]
pub struct GridSpace {
    entity: GridEntity,
}

#[derive(Clone)]
pub enum GridEntity {
    Wall,
    Empty,
}

impl Default for GridSpace {
    fn default() -> Self {
        Self {
            entity: GridEntity::Empty,
        }
    }
}

pub fn populate_grid<G: Grid<Item = GridSpace>>(board: &str) -> G {
    let lines: Vec<_> = board.split("\n").map(|line| line.trim()).collect();

    assert!(lines.len() > 0, "Map must contain at least one line");

    let width = lines[0].len();

    let r = lines
        .iter()
        .enumerate()
        .find(|&(_, &line)| line.len() != width);

    if let Some((line_num, line)) = r {
        panic!(
            "First line established the line width as {}, but line {} was {} characters",
            width,
            line_num + 1,
            line.len()
        );
    }

    let mut g = G::with_size(width, lines.len());

    for (y, &line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                'x' => g.set(
                    x,
                    y,
                    GridSpace {
                        entity: GridEntity::Wall,
                    },
                ),
                '.' => {}
                _ => {}
            }
        }
    }

    g
}

impl Display for GridSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self.entity {
            GridEntity::Empty => ". ",
            GridEntity::Wall => "X ",
        };

        write!(f, "{}", char)
    }
}

#[cfg(test)]
mod tests {}

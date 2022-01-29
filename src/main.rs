mod game;
mod grid;

use std::fmt::Display;

use game::GridSpace;
use grid::{Grid, Grid1};

const t0: &'static str = include_str!("../test_grids/spiral.txt");

fn main() {
    let g = game::populate_grid::<Grid1<_>>(t0);

    println!("{}", g);
}

impl Display for Grid1<GridSpace> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(f, "{}", self.get(x, y))?;
            }
            write!(f, "\n")?;
        }

        std::fmt::Result::Ok(())
    }
}

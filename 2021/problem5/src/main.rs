use anyhow::{anyhow, bail, Context, Result};
use scan_fmt::scan_fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

const GRID_SIZE: usize = 1000;

type GridRow = Vec<usize>;
type GridValues = Vec<GridRow>;

struct Grid {
    values: GridValues,
}

impl Grid {
    fn new() -> Self {
        Grid {
            values: vec![vec![0; GRID_SIZE]; GRID_SIZE],
        }
    }

    fn add_line(
        &mut self,
        x_1: usize,
        y_1: usize,
        x_2: usize,
        y_2: usize,
        ignore_diagonals: bool,
    ) -> Result<()> {
        if ignore_diagonals && x_1 != x_2 && y_1 != y_2 {
            return Ok(());
        }

        let horizontal_range: Box<dyn Iterator<Item = _>> = if x_1 > x_2 {
            Box::new((x_2..=x_1).rev())
        } else {
            Box::new(x_1..=x_2)
        };
        let vertical_range: Box<dyn Iterator<Item = _>> = if y_1 > y_2 {
            Box::new((y_2..=y_1).rev())
        } else {
            Box::new(y_1..=y_2)
        };

        let normal_diagonal = if x_2 > x_1 { x_2 - x_1 } else { x_1 - x_2 }
            == if y_2 > y_1 { y_2 - y_1 } else { y_1 - y_2 };

        if y_1 == y_2 {
            for x in horizontal_range {
                *(self
                    .values
                    .get_mut(y_1)
                    .ok_or_else(|| {
                        anyhow!(
                            "Vertical coordinate {} in {},{} -> {},{} exceeds grid size.",
                            y_1, x_1, y_1, x_2, y_2
                        )
                    })?
                    .get_mut(x)
                    .ok_or_else(|| {
                        anyhow!(
                            "Horizontal coordinate {} in {},{} -> {},{} exceeds grid size.",
                            x, x_1, y_1, x_2, y_2
                        )
                    })?) += 1;
            }
        } else if x_1 == x_2 {
            for y in vertical_range {
                *(self
                    .values
                    .get_mut(y)
                    .ok_or_else(|| {
                        anyhow!(
                            "Vertical coordinate {} in {},{} -> {},{} exceeds grid size.",
                            y, x_1, y_1, x_2, y_2)
                    })?
                    .get_mut(x_1)
                    .ok_or_else(|| {
                        anyhow!(
                            "Horizontal coordinate {} in {},{} -> {},{} exceeds grid size.",
                            x_1, x_1, y_1, x_2, y_2
                        )
                    })?) += 1;
            }
        } else if normal_diagonal {
            for (y, x) in vertical_range.zip(horizontal_range) {
                *(self
                    .values
                    .get_mut(y)
                    .ok_or_else(|| {
                        anyhow!(
                            "Vertical coordinate {} in {},{} -> {},{} exceeds grid size.",
                            y, x_1, y_1, x_2, y_2
                        )
                    })?
                    .get_mut(x)
                    .ok_or_else(|| {
                        anyhow!(
                            "Horizontal coordinate {} in {},{} -> {},{} exceeds grid size.",
                            x_1, x_1, y_1, x_2, y_2
                        )
                    })?) += 1;
            }
        } else {
            bail!(
                "Unrecognized line type in {},{} -> {},{}.",
                x_1,
                y_1,
                x_2,
                y_2
            );
        }

        Ok(())
    }

    fn overlaps(&self) -> usize {
        let mut overlaps = 0;
        for row in self.values.iter() {
            for &value in row.iter() {
                if value > 1 {
                    overlaps += 1;
                }
            }
        }
        overlaps
    }
}

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or(anyhow!("No filename provided."))?;
    let file = File::open(filename).context("Failed to open file.")?;
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .context("Failed to read line.")?;

    let mut grid = Grid::new();
    for line in lines.iter() {
        if let Ok((x_1, y_1, x_2, y_2)) =
            scan_fmt!(line, "{d},{d} -> {d},{d}", usize, usize, usize, usize)
        {
            grid.add_line(x_1, y_1, x_2, y_2, false)?;
        } else {
            bail!("Line invalid: {}", line);
        }
    }

    println!("Overlaps: {}", grid.overlaps());

    Ok(())
}

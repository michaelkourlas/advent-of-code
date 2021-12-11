use anyhow::{anyhow, bail, Context, Result};
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Octopus {
    energy: usize,
    flashed: bool,
}

struct Octopuses {
    grid: Vec<Vec<Octopus>>,
    total_flash_count: usize,
    step_flash_count: usize,
}

impl Octopuses {
    fn process_flash(&mut self, x: usize, y: usize) {
        if self.grid[y][x].energy <= 9 || self.grid[y][x].flashed {
            return;
        }

        self.grid[y][x].flashed = true;
        self.total_flash_count += 1;
        self.step_flash_count += 1;

        let mut adjacents = vec![(x + 1, y), (x + 1, y + 1), (x, y + 1)];
        if y > 0 {
            adjacents.push((x, y - 1));
            adjacents.push((x + 1, y - 1));
        }
        if x > 0 {
            adjacents.push((x - 1, y + 1));
            adjacents.push((x - 1, y));
        }
        if y > 0 && x > 0 {
            adjacents.push((x - 1, y - 1));
        }

        for adjacent in adjacents {
            let octopus =
                match self.grid.get_mut(adjacent.1).and_then(|row| row.get_mut(adjacent.0)) {
                    Some(octopus) => octopus,
                    None => continue,
                };

            octopus.energy += 1;
            self.process_flash(adjacent.0, adjacent.1);
        }
    }

    fn step(&mut self) {
        self.step_flash_count = 0;

        for row in &mut self.grid {
            for octopus in row {
                octopus.energy += 1
            }
        }

        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                self.process_flash(x, y)
            }
        }

        for row in &mut self.grid {
            for octopus in row {
                if octopus.flashed {
                    octopus.energy = 0;
                    octopus.flashed = false;
                }
            }
        }
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

    let mut octopuses = Octopuses {
        grid: lines
            .iter()
            .map(|s| {
                s.chars()
                    .map(|c| {
                        c.to_string()
                            .parse::<usize>()
                            .map(|v| Octopus {
                                energy: v,
                                flashed: false,
                            })
                            .with_context(|| {
                                anyhow!(
                                    "Failed to parse {} as unsigned integer.",
                                    c
                                )
                            })
                    })
                    .collect::<Result<_>>()
            })
            .collect::<Result<_>>()?,
        total_flash_count: 0,
        step_flash_count: 0
    };

    let mut i = 0;
    while octopuses.step_flash_count != octopuses.grid.len() * octopuses.grid[0].len() {
        octopuses.step();
        i += 1;
        if i == 100 {
            println!("Flash count at i = 100: {}", octopuses.total_flash_count);
        }
    }

    println!("All flash step: {}", i);

    Ok(())
}

use anyhow::{anyhow, Context, Result};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn left(&self) -> Option<Self> {
        match self.x {
            0 => None,
            _ => Some(Coordinates {
                x: self.x - 1,
                y: self.y,
            }),
        }
    }

    fn up(&self) -> Option<Self> {
        match self.y {
            0 => None,
            _ => Some(Coordinates {
                x: self.x,
                y: self.y - 1,
            }),
        }
    }

    fn right(&self) -> Self {
        Coordinates {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn down(&self) -> Self {
        Coordinates {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn adjacents(&self) -> Vec<Self> {
        let mut adjacents = Vec::new();
        if let Some(left) = self.left() {
            adjacents.push(left);
        }
        if let Some(up) = self.up() {
            adjacents.push(up);
        }
        adjacents.push(self.right());
        adjacents.push(self.down());
        adjacents
    }
}

struct HeightMap {
    grid: Vec<Vec<usize>>,
}

impl HeightMap {
    fn get_height(&self, coords: Coordinates) -> Option<usize> {
        self.grid
            .get(coords.y)
            .and_then(|row| row.get(coords.x))
            .copied()
    }

    fn coords(&self) -> impl Iterator<Item = Coordinates> {
        let y_size = self.grid.len();
        let x_size = self.grid[0].len();
        return (0..y_size)
            .map(move |y| (0..x_size).map(move |x| Coordinates { x, y }))
            .flatten();
    }
}

type Heights = HashMap<Coordinates, usize>;

fn get_adjacent_heights(height_map: &HeightMap, coords: Coordinates) -> Heights {
    let mut heights = HashMap::new();
    for adjacent_coords in coords.adjacents() {
        if let Some(height) = height_map.get_height(adjacent_coords) {
            heights.insert(adjacent_coords, height);
        }
    }
    return heights;
}

fn is_low_point(height_map: &HeightMap, coords: Coordinates) -> Result<bool> {
    if let Some(height) = height_map.get_height(coords) {
        let adjacent_heights = get_adjacent_heights(height_map, coords);
        for &adjacent_height in adjacent_heights.values() {
            if adjacent_height <= height {
                return Ok(false);
            }
        }
        return Ok(true);
    }
    Ok(false)
}

fn get_basin_size(
    height_map: &HeightMap,
    coords: Coordinates,
    basin_coords: &mut Vec<Coordinates>,
) -> usize {
    let height = match height_map.get_height(coords) {
        Some(height) => height,
        None => return 0,
    };

    let mut basin_size = 1;
    for adjacent_coords in coords.adjacents() {
        if basin_coords.contains(&adjacent_coords) {
            continue;
        }

        let adjacent_height = match height_map.get_height(adjacent_coords) {
            Some(adjacent_height) => adjacent_height,
            None => continue,
        };

        if adjacent_height >= height && adjacent_height != 9 {
            basin_coords.push(adjacent_coords);
            basin_size += get_basin_size(height_map, adjacent_coords, basin_coords);
        }
    }

    return basin_size;
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

    let height_map: HeightMap = HeightMap {
        grid: lines
            .iter()
            .map(|s| {
                s.chars()
                    .map(|c| {
                        c.to_string().parse::<usize>().with_context(|| {
                            anyhow!(
                                "Failed to parse {:?} as unsigned integer.",
                                c
                            )
                        })
                    })
                    .collect::<Result<_>>()
            })
            .collect::<Result<_>>()
            .context("Failed to parse map.")?,
    };

    let mut risk_sum = 0;
    let mut basin_sizes = Vec::new();
    for coords in height_map.coords() {
        if is_low_point(&height_map, coords)? {
            risk_sum += height_map.get_height(coords).unwrap() + 1;

            let mut basin_coords = vec![coords];
            basin_sizes.push(get_basin_size(&height_map, coords, &mut basin_coords));
        }
    }

    basin_sizes.sort();

    println!("Risk sum: {}", risk_sum);
    println!("Basin product: {}", basin_sizes.iter().rev().take(3).product::<usize>());

    Ok(())
}

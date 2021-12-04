use anyhow::{anyhow, bail, Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

const BOARD_SIZE: usize = 5;

#[derive(Copy, Clone, Debug)]
struct BoardValue {
    value: usize,
    marked: bool,
}

impl BoardValue {
    fn new(v: usize) -> BoardValue {
        BoardValue {
            value: v,
            marked: false,
        }
    }
}

type BoardRow = [BoardValue; BOARD_SIZE];
type BoardValues = [BoardRow; BOARD_SIZE];

#[derive(Copy, Clone)]
struct Board {
    values: BoardValues,
}

impl Board {
    fn parse(rows: &[&str]) -> Result<Board> {
        if rows.len() != BOARD_SIZE {
            bail!("Invalid number of rows {}.", rows.len());
        }

        let mut board_values: Vec<BoardRow> = Vec::new();
        for &row in rows.iter() {
            let row_numbers: Vec<usize> = row
                .split(' ')
                .filter(|&s| s != "")
                .map(|s| {
                    s.parse().with_context(|| {
                        format!("Failed to parse '{}' as integer.", s)
                    })
                })
                .collect::<Result<_>>()?;
            board_values.push(
                row_numbers
                    .iter()
                    .map(|&n| BoardValue::new(n))
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            );
        }

        Ok(Board {
            values: board_values.try_into().unwrap(),
        })
    }

    fn mark(&mut self, value: usize) {
        for row in self.values.iter_mut() {
            for board_value in row.iter_mut() {
                if board_value.value == value {
                    board_value.marked = true;
                }
            }
        }
    }

    fn complete(&self) -> bool {
        // Check for completed columns.
        for i in 0..BOARD_SIZE {
            if self.values.iter().map(|c| c[i]).all(|bv| bv.marked) {
                return true;
            }
        }

        // Check for completed rows.
        for row in self.values.iter() {
            if row.iter().all(|bv| bv.marked) {
                return true;
            }
        }

        return false;
    }

    fn sum_unmarked(&self) -> usize {
        self.values
            .iter()
            .map(|row| {
                row.iter()
                    .filter(|bv| !bv.marked)
                    .map(|bv| bv.value)
                    .sum::<usize>()
            })
            .sum()
    }
}

impl std::fmt::Display for Board {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        for row in self.values {
            for value in row {
                write!(
                    f,
                    "{}{}\t",
                    value.value,
                    if value.marked { "*" } else { "" }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
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
        .context("Failed to read line.")?
        .into_iter()
        .filter(|s| !s.is_empty())
        .collect();

    let drawn_numbers: Vec<usize> = lines
        .iter()
        .nth(0)
        .ok_or(anyhow!("Missing drawn numbers."))?
        .split(",")
        .map(|s| {
            s.parse()
                .with_context(|| format!("Failed to parse {} as integer.", s))
        })
        .collect::<Result<_>>()?;

    let mut boards = Vec::new();
    for chunk in lines
        .iter()
        .skip(1)
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .chunks(BOARD_SIZE)
    {
        boards
            .push(Board::parse(chunk).context("Failed to parse bingo board.")?);
    }

    for number in drawn_numbers {
        for board in boards.iter_mut() {
            if board.complete() {
                continue;
            }

            board.mark(number);
            if board.complete() {
                println!(
                    "The following board wins when {} is drawn, \
                     with a score of {}:",
                    number,
                    board.sum_unmarked() * number
                );
                println!("{}", board);
            }
        }
    }

    Ok(())
}

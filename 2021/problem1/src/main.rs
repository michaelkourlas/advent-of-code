use anyhow::{anyhow, Context, Result};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or(anyhow!("No filename provided."))?;
    let file = File::open(filename).context("Failed to open file.")?;
    let reader = BufReader::new(file);

    let mut previous = Option::<u32>::None;
    let mut previous_increases = 0;

    let mut window = VecDeque::new();
    let mut window_increases = 0;
    const WINDOW_SIZE: usize = 3;

    for line in reader.lines() {
        let line = line.context("Failed to read line.")?;
        let curr = line.parse::<u32>().with_context(|| {
            format!("Failed to parse '{}' as unsigned integer.", line)
        })?;

        if let Some(prev) = previous {
            if curr > prev {
                previous_increases += 1;
            }
        }
        if window.len() == WINDOW_SIZE {
            let prev_window_sum = window.iter().sum::<u32>();
            let curr_window_sum = window.iter().skip(1).sum::<u32>() + curr;
            if curr_window_sum > prev_window_sum {
                window_increases += 1;
            }
        }

        if window.len() == WINDOW_SIZE {
            window.pop_front();
        }
        window.push_back(curr);
        previous = Some(curr);
    }
    println!("Number of measurement increases: {}", previous_increases);
    println!(
        "Number of measurement increases (window of size {}): {}",
        WINDOW_SIZE, window_increases
    );

    Ok(())
}

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

    let mut prev: Option<u32> = None;
    let mut prev_incrs = 0;

    let mut window = VecDeque::new();
    let mut window_incrs = 0;
    const WINDOW_SIZE: usize = 3;

    for line in reader.lines() {
        let line = line.context("Failed to read line.")?;
        let curr = line.parse::<u32>().with_context(|| {
            format!("Failed to parse '{}' as unsigned integer.", line)
        })?;

        if let Some(prev) = prev {
            if curr > prev {
                prev_incrs += 1;
            }
        }
        if window.len() == WINDOW_SIZE {
            let prev_sum = window.iter().sum::<u32>();
            let curr_sum = window.iter().skip(1).sum::<u32>() + curr;
            if curr_sum > prev_sum {
                window_incrs += 1;
            }
        }

        if window.len() == WINDOW_SIZE {
            window.pop_front();
        }
        window.push_back(curr);
        prev = Some(curr);
    }
    println!("Increases (relative to previous): {}", prev_incrs);
    println!(
        "Increases (window of size {}): {}",
        WINDOW_SIZE, window_incrs
    );

    Ok(())
}

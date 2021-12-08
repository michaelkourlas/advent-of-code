use anyhow::{anyhow, Context, Result};

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or(anyhow!("No filename provided."))?;
    let line =
        std::fs::read_to_string(filename).context("Failed to open file.")?;
    let positions = line
        .trim_end()
        .split(",")
        .map(|t| {
            t.parse::<usize>()
                .with_context(|| format!("Failed to parse {} as integer.", t))
        })
        .collect::<Result<Vec<_>>>()?;

    let max_position = *positions.iter().max().context("No data provided.")?;

    let mut best_position = 0;
    let mut best_fuel = usize::MAX;
    for i in 0..=max_position {
        let required_fuel: usize = positions
            .iter()
            .map(|&p| if p > i { p - i } else { i - p })
            .map(|f| (f * (f + 1)) / 2) // Triangular number formula for part 2
            .sum();
        if required_fuel < best_fuel {
            best_fuel = required_fuel;
            best_position = i;
        }
    }

    println!("Fuel required: {} ({})", best_fuel, best_position);

    Ok(())
}

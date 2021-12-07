use std::collections::VecDeque;

use anyhow::{anyhow, bail, Context, Result};

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or(anyhow!("No filename provided."))?;
    let line =
        std::fs::read_to_string(filename).context("Failed to open file.")?;
    let initial_timers = line
        .trim_end()
        .split(",")
        .map(|t| {
            t.parse::<usize>()
                .with_context(|| format!("Failed to parse {} as integer.", t))
        })
        .collect::<Result<Vec<_>>>()?;

    const MAX_TIMER_VALUE: usize = 8;
    let mut timer_value_count_map =
        VecDeque::from(vec![0; MAX_TIMER_VALUE + 1]);
    for initial_timer in initial_timers {
        if initial_timer > MAX_TIMER_VALUE {
            bail!("{} exceeded maximum timer value.", initial_timer);
        }
        timer_value_count_map[initial_timer] += 1;
    }

    const SIMULATION_DAYS: usize = 256;
    for _ in 0..SIMULATION_DAYS {
        let num_expired_timers = timer_value_count_map.pop_front().unwrap();
        *(timer_value_count_map.get_mut(6).unwrap()) += num_expired_timers;
        timer_value_count_map.push_back(num_expired_timers);
    }

    println!(
        "Fish count: {}",
        timer_value_count_map.iter().sum::<usize>()
    );

    Ok(())
}

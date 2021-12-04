use anyhow::{anyhow, bail, Context, Result};
use scan_fmt::scan_fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or(anyhow!("No filename provided."))?;
    let file = File::open(filename).context("Failed to open file.")?;
    let reader = BufReader::new(file);

    let mut x = 0;
    let mut y = 0;

    let mut x_aim = 0;
    let mut y_aim = 0;
    let mut aim = 0;

    for line in reader.lines() {
        let line = line.context("Failed to read line.")?;
        if let Ok((direction, distance)) =
            scan_fmt!(&line, "{} {d}", String, u32)
        {
            match direction.as_str() {
                "up" => {
                    y -= distance;
                    aim -= distance;
                }
                "down" => {
                    y += distance;
                    aim += distance;
                }
                "forward" => {
                    x += distance;
                    x_aim += distance;
                    y_aim += aim * distance;
                }
                _ => bail!("Unrecognized direction {}", direction),
            }
        } else {
            bail!("Unrecognized command {}", line);
        }
    }

    println!("x * y = {}", x * y);
    println!("x_aim * y_aim = {}", x_aim * y_aim);

    Ok(())
}

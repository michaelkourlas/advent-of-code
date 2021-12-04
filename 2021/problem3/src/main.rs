use anyhow::{anyhow, bail, Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn most_common_bit(bits: &[char]) -> char {
    // If bits are equally common, return 1.
    let count = bits.iter().filter(|&&b| b == '1').count();
    if count >= bits.len() - count {
        '1'
    } else {
        '0'
    }
}

fn least_common_bit(bits: &[char]) -> char {
    // If bits are equally common, return 0.
    let count = bits.iter().filter(|&&b| b == '0').count();
    println!("{}, {}", bits.len(), count);
    if count <= bits.len() - count {
        '0'
    } else {
        '1'
    }
}

fn main() -> Result<()> {
    let filename = std::env::args()
        .nth(1)
        .ok_or(anyhow!("No filename provided."))?;
    let num_bits_str = std::env::args()
        .nth(2)
        .ok_or(anyhow!("No number of bits provided."))?;
    let num_bits = num_bits_str.parse::<usize>().with_context(|| {
        format!("Failed to parse '{}' as unsigned integer.", num_bits_str)
    })?;
    let file = File::open(filename).context("Failed to open file.")?;
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .collect::<Result<Vec<String>, _>>()
        .context("Failed to read line.")?;

    for line in lines.iter() {
        if line.chars().any(|c| c != '0' && c != '1') {
            bail!("{} is not a binary number.", line);
        }
        if line.len() != num_bits {
            bail!("Unexpected number of bits in {}.", line);
        }
    }

    // Calculate gamma and epsilon.
    let mut gamma = 0;
    let mut epsilon = 0;
    for i in 0..num_bits {
        gamma *= 2;
        epsilon *= 2;

        let bits_in_ith_position: Vec<char> =
            lines.iter().map(|s| s.chars().nth(i).unwrap()).collect();
        if most_common_bit(&bits_in_ith_position) == '1' {
            gamma += 1;
        }
        if least_common_bit(&bits_in_ith_position) == '1' {
            epsilon += 1;
        }
    }

    // Calculate O2 and CO2 ratings.
    let mut o2_values: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    for i in 0..num_bits {
        if o2_values.len() > 1 {
            let o2_bit = most_common_bit(
                &o2_values
                    .iter()
                    .map(|s| s.chars().nth(i).unwrap())
                    .collect::<Vec<char>>(),
            );
            o2_values.retain(|s| s.chars().nth(i) == Some(o2_bit));
        } else {
            break;
        }
    }
    let o2_rating = usize::from_str_radix(o2_values[0], 2)?;

    let mut co2_values: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
    for i in 0..num_bits {
        if co2_values.len() > 1 {
            let co2_bit = least_common_bit(
                &co2_values
                    .iter()
                    .map(|s| s.chars().nth(i).unwrap())
                    .collect::<Vec<char>>(),
            );
            co2_values.retain(|v| v.chars().nth(i) == Some(co2_bit));
        } else {
            break;
        }
    }
    let co2_rating = usize::from_str_radix(co2_values[0], 2)?;

    println!("gamma * epsilon: {}", gamma * epsilon);
    println!("o2_rating * co2_rating: {}", o2_rating * co2_rating);

    Ok(())
}

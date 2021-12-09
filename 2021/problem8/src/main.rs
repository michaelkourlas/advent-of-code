use anyhow::{anyhow, bail, Context, Result};
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn to_digit(segments: &str) -> Result<usize> {
    let mut chars: Vec<char> = segments.chars().collect();
    chars.sort();
    let segments = String::from_iter(chars.iter());
    match String::from_iter(chars.iter()).as_str() {
        "abcefg" => Ok(0),
        "cf" => Ok(1),
        "acdeg" => Ok(2),
        "acdfg" => Ok(3),
        "bcdf" => Ok(4),
        "abdfg" => Ok(5),
        "abdefg" => Ok(6),
        "acf" => Ok(7),
        "abcdefg" => Ok(8),
        "abcdfg" => Ok(9),
        _ => bail!("Invalid segments {}", segments),
    }
}

fn parse_patterns(patterns: &[&str]) -> Result<HashMap<char, char>> {
    let mut map = HashMap::new();

    // Identify a. a is the only character present in the three-pattern but not
    // the two-pattern.
    let two_pattern = patterns
        .iter()
        .copied()
        .filter(|p| p.len() == 2)
        .next()
        .ok_or_else(|| anyhow!("Expected pattern of length 2."))?;
    let three_pattern = patterns
        .iter()
        .copied()
        .filter(|p| p.len() == 3)
        .next()
        .ok_or_else(|| anyhow!("Expected pattern of length 3."))?;
    let line_a = three_pattern
        .chars()
        .filter(|c| !two_pattern.contains(*c))
        .next()
        .ok_or_else(|| anyhow!("Failed to identify a."))?;
    map.insert(line_a, 'a');

    // Identify c. c is the only character present in the two-pattern which is
    // present in exactly two of the six-patterns.
    let six_patterns: Vec<&str> =
        patterns.iter().copied().filter(|p| p.len() == 6).collect();
    let line_c = two_pattern
        .chars()
        .filter(|c| {
            six_patterns
                .iter()
                .copied()
                .filter(|p| p.contains(*c))
                .count()
                == 2
        })
        .next()
        .ok_or_else(|| anyhow!("Failed to identify c."))?;
    map.insert(line_c, 'c');

    // Identify f. f is the character in the two-pattern that is not c.
    let line_f = two_pattern
        .chars()
        .filter(|&c| c != line_c)
        .next()
        .ok_or_else(|| anyhow!("Failed to identify f."))?;
    map.insert(line_f, 'f');

    // Identify e. e is the only character not common to all of the
    // five-patterns that is paired with c and that is not f.
    let five_patterns: Vec<&str> =
        patterns.iter().copied().filter(|p| p.len() == 5).collect();
    let five_pattern_common_chars: Vec<char> = "abcdefg"
        .chars()
        .filter(|c| five_patterns.iter().copied().all(|p| p.contains(*c)))
        .collect();
    let line_e = "abcdefg"
        .chars()
        .filter(|c| five_patterns.iter().copied().any(|p| p.contains(*c)))
        .filter(|c| !five_pattern_common_chars.contains(c))
        .filter(|&c| c != line_c && c != line_f)
        .filter(|c| {
            five_patterns
                .iter()
                .copied()
                .filter(|p| p.contains(line_c))
                .any(|p| p.contains(*c))
        })
        .next()
        .ok_or_else(|| anyhow!("Failed to identify e."))?;
    map.insert(line_e, 'e');

    // Identify b. b is the only character not common to all of the
    // five-patterns that is paired with f and that is not c.
    let line_b = "abcdefg"
        .chars()
        .filter(|c| five_patterns.iter().copied().any(|p| p.contains(*c)))
        .filter(|c| !five_pattern_common_chars.contains(c))
        .filter(|&c| c != line_c && c != line_f)
        .filter(|c| {
            five_patterns
                .iter()
                .copied()
                .filter(|p| p.contains(line_f))
                .any(|p| p.contains(*c))
        })
        .next()
        .ok_or_else(|| anyhow!("Failed to identify b."))?;
    map.insert(line_b, 'b');

    // Identify d. d is the character in the four-pattern that is not b, c, or
    // f.
    let four_pattern = patterns
        .iter()
        .copied()
        .filter(|p| p.len() == 4)
        .next()
        .ok_or_else(|| anyhow!("Expected pattern of length 2."))?;
    let line_d = four_pattern
        .chars()
        .filter(|&c| c != line_b && c != line_c && c != line_f)
        .next()
        .ok_or_else(|| anyhow!("Unable to identify d."))?;
    map.insert(line_d, 'd');

    // The sole remaining character is g.
    let line_g = "abcdefg"
        .chars()
        .filter(|&c| {
            c != line_a
                && c != line_b
                && c != line_c
                && c != line_d
                && c != line_e
                && c != line_f
        })
        .next()
        .ok_or_else(|| anyhow!("Failed to identify g."))?;
    map.insert(line_g, 'g');

    Ok(map)
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

    let mut unique_digits_frequency = 0;
    let mut total = 0;
    for line in lines {
        let input = line
            .trim_end()
            .split("|")
            .map(|a| a.trim().split(" ").collect::<Vec<_>>())
            .collect::<Vec<_>>();
        if let [patterns, output] = &input[..] {
            let pattern_map = parse_patterns(patterns)?;

            let mut digits = Vec::new();
            for broken_segments in output.iter() {
                let mut fixed_segments_chars = broken_segments
                    .chars()
                    .map(|c| {
                        pattern_map.get(&c).ok_or_else(|| {
                            anyhow!("Unrecognized character {}", c)
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;
                fixed_segments_chars.sort();
                let fixed_segments: String =
                    fixed_segments_chars.into_iter().collect();
                digits.push(to_digit(fixed_segments.as_str())?);
            }

            for &digit in digits.iter() {
                if digit == 1 || digit == 4 || digit == 7 || digit == 8 {
                    unique_digits_frequency += 1;
                }
            }

            let mut num = 0;
            for digit in digits {
                num *= 10;
                num += digit;
            }
            total += num;
        } else {
            bail!("Invalid input.")
        }
    }

    println!("1,4,7,8: {}", unique_digits_frequency);
    println!("Total: {}", total);

    Ok(())
}

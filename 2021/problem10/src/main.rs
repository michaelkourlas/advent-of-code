use anyhow::{anyhow, bail, Context, Result};
use std::{
    collections::VecDeque,
    fs::File,
    io::{BufRead, BufReader},
};

fn get_closing_brace(c: char) -> Result<char> {
    match c {
        '(' => Ok(')'),
        '[' => Ok(']'),
        '{' => Ok('}'),
        '<' => Ok('>'),
        _ => bail!("Unexpected character {}.", c)
    }
}

fn get_error_score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => 0
    }
}

fn get_completion_score(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0
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

    let mut total_error_score = 0;
    let mut completion_scores = Vec::new();
    for line in lines {
        let mut stack = VecDeque::new();

        let mut error_score = 0;
        for c in line.chars() {
            if let Ok(_) = get_closing_brace(c) {
                stack.push_front(c);
            } else if let Some(opening_char) = stack.pop_front() {
                let expected_closing_char = get_closing_brace(opening_char)?;
                if c != expected_closing_char {
                    error_score = get_error_score(c);
                    total_error_score += error_score;
                }
            }
        }

        if error_score == 0 {
            let mut completion_score = 0;
            for c in stack.iter().copied() {
                completion_score *= 5;
                completion_score += get_completion_score(get_closing_brace(c)?);
            }
            completion_scores.push(completion_score);
        }
    }

    completion_scores.sort();
    let completion_score = completion_scores[completion_scores.len() / 2];

    println!("Error score: {}", total_error_score);
    println!("Completion score: {}", completion_score);

    Ok(())
}

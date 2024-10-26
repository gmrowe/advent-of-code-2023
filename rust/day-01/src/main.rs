use std::{env, fs, io};

fn parse_literal_digit(s: &str) -> Option<u32> {
    const RADIX: u32 = 10;
    s.starts_with(|c: char| c.is_ascii_digit()).then(|| {
        s.chars()
            .next()
            .and_then(|c| c.to_digit(RADIX))
            .expect("slice starts with a digit")
    })
}

fn extract_first_and_last_digits<F>(s: &str, parse_fn: F) -> u32
where
    F: Fn(&str) -> Option<u32>,
{
    let digits = (0..s.len())
        .filter_map(|i| parse_fn(&s[i..]))
        .collect::<Vec<_>>();
    let first = digits.first().expect("Line has at least 1 digit");
    let last = digits.last().expect("Line has at least 1 digit");
    10 * first + last
}

fn part_01(input: &str) -> String {
    input
        .lines()
        .map(|s| extract_first_and_last_digits(s, parse_literal_digit))
        .sum::<u32>()
        .to_string()
}

fn parse_text_digit(s: &str) -> Option<u32> {
    let text_digits = [
        "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    text_digits
        .into_iter()
        .enumerate()
        .skip(1)
        .find(|(_i, text)| s.starts_with(text))
        .map(|(i, _text)| i as u32)
}

fn parse_literal_or_text_digit(s: &str) -> Option<u32> {
    parse_literal_digit(s).or_else(|| parse_text_digit(s))
}

fn part_02(input: &str) -> String {
    input
        .lines()
        .map(|s| extract_first_and_last_digits(s, parse_literal_or_text_digit))
        .sum::<u32>()
        .to_string()
}

enum AOCErr {
    NoInputProvided,
    CannotReadFile(io::Error),
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let program = &args[0];

    let input = args
        .get(1)
        .ok_or(AOCErr::NoInputProvided)
        .and_then(|path| fs::read_to_string(path).map_err(AOCErr::CannotReadFile));

    match input {
        Ok(s) => println!(
            "[advent-of-code-2023:day_01:part_01] {result_01}\n\
             [advent-of-code-2023:day_01:part_02] {result_02}",
            result_01 = part_01(&s),
            result_02 = part_02(&s)
        ),
        Err(AOCErr::NoInputProvided) => eprintln!("Usage: {program} <input_filename>"),
        Err(AOCErr::CannotReadFile(reason)) => eprintln!("ERROR Could not read input: {reason}"),
    }
}

#![allow(unused)]

use std::{env, fs, io};

enum AOCErr {
    NoInputProvided,
    CannotReadFile(io::Error),
}

fn err_msg(err: &AOCErr, program: &str) -> String {
    match err {
        AOCErr::NoInputProvided => format!("Usage: {program} <input_filename>"),
        AOCErr::CannotReadFile(reason) => format!("Could not read input: {reason}"),
    }
}

fn main() -> Result<(), String> {
    let args = env::args().collect::<Vec<String>>();
    let program = &args[0];

    let input = args
        .get(1)
        .ok_or(AOCErr::NoInputProvided)
        .and_then(|path| fs::read_to_string(path).map_err(AOCErr::CannotReadFile))
        .map_err(|err| err_msg(&err, program))?;

    let result_01 = part_01(&input);
    println!("[advent-of-code-2023:day_04:part_01] {result_01}");
    let result_02 = part_02(&input);
    println!("[advent-of-code-2023:day_04:part_02] {result_02}");
    Ok(())
}

fn part_01(input: &str) -> String {
    input
        .lines()
        .map(line_to_i32s)
        .map(|v| predict_next(&v))
        .sum::<i32>()
        .to_string()
}

fn part_02(input: &str) -> String {
    "TODO".to_string()
}

fn line_to_i32s(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

fn predict_next(seq: &[i32]) -> i32 {
    if seq.iter().all(|n| *n == 0) {
        return 0;
    }

    let diffs = seq.windows(2).map(|d| d[1] - d[0]).collect::<Vec<_>>();
    seq.last().unwrap() + predict_next(&diffs)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn predict_single_zero_constant_sequence() {
        let seq = [0];
        assert_eq!(predict_next(&seq), 0);
    }

    #[test]
    fn predict_multi_zero_constant_sequence() {
        let seq = [0, 0, 0, 0, 0];
        assert_eq!(predict_next(&seq), 0);
    }

    #[test]
    fn predict_constant_sequence_1s() {
        let seq = [1, 1];
        assert_eq!(predict_next(&seq), 1);
    }

    #[test]
    fn predict_monotonically_increasing_series() {
        // 1     2     3
        //    1     1
        //       0
        let seq = [1, 2, 3];
        assert_eq!(predict_next(&seq), 4);
    }

    #[test]
    fn predict_non_constant_increasing_series() {
        let seq = [1, 3, 6, 10, 15, 21];
        assert_eq!(predict_next(&seq), 28);
    }

    #[test]
    fn predict_non_obvious_increasing_series() {
        let seq = [10, 13, 16, 21, 30, 45];
        assert_eq!(predict_next(&seq), 68);
    }
}

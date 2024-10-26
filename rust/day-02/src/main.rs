use std::{env, fs, io};

#[derive(Debug, Clone)]
struct Game {
    id: usize,
    reds: Vec<u32>,
    greens: Vec<u32>,
    blues: Vec<u32>,
}

fn parse_game(line: &str) -> Game {
    let (id, trials) = line
        .split_once(':')
        .expect("Every line has the form <game_id>: <trials>");
    let id_num = id
        .split_once(' ')
        .expect("Every id has the form 'Game <num>'")
        .1
        .parse::<usize>()
        .expect("The num can be parsed to a usize");
    let mut games = Game {
        reds: Vec::new(),
        greens: Vec::new(),
        blues: Vec::new(),
        id: id_num,
    };
    for trial in trials.trim().split("; ") {
        games.reds.push(0);
        games.greens.push(0);
        games.blues.push(0);
        // Trials are a comma separated list of pairs
        for pair in trial.split(", ") {
            let (count, color) = pair
                .split_once(' ')
                .expect("Every pair has the form <count> <color>");
            let n = count.parse::<u32>().expect("The count is a valid number");
            match color {
                "red" => *games.reds.last_mut().expect("nonempty slice") += n,
                "green" => *games.greens.last_mut().expect("nonempty slice") += n,
                "blue" => *games.blues.last_mut().expect("nonempty slice") += n,
                _ => panic!("[ERROR] Unknown color: {color}"),
            }
        }
    }
    games
}

fn part_01(input: &str) -> String {
    input
        .lines()
        .map(parse_game)
        .filter(|g| {
            g.reds.iter().all(|n| *n <= 12)
                && g.greens.iter().all(|n| *n <= 13)
                && g.blues.iter().all(|n| *n <= 14)
        })
        .map(|g| g.id)
        .sum::<usize>()
        .to_string()
}

fn part_02(input: &str) -> String {
    input
        .lines()
        .map(parse_game)
        .map(|g| {
            let max_reds = g.reds.iter().max().unwrap_or(&0);
            let max_blues = g.blues.iter().max().unwrap_or(&0);
            let max_greens = g.greens.iter().max().unwrap_or(&0);
            max_reds * max_blues * max_greens
        })
        .sum::<u32>()
        .to_string()
}

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

    println!(
        "[advent-of-code-2023:day_02:part_01] {result_01}\n\
         [advent-of-code-2023:day_02:part_02] {result_02}",
        result_01 = part_01(&input),
        result_02 = part_02(&input)
    );
    Ok(())
}

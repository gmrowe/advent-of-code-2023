use std::collections::HashSet;
use std::str::FromStr;
use std::{env, fs, io};

struct GameCard {
    player_numbers: Vec<u32>,
    winning_numbers: Vec<u32>,
    id: usize,
}

impl GameCard {
    fn match_count(&self) -> usize {
        let player = self.player_numbers.iter().collect::<HashSet<_>>();
        let winning = self.winning_numbers.iter().collect::<HashSet<_>>();
        player.intersection(&winning).count()
    }

    fn score(&self) -> u32 {
        let matching = self.match_count();
        if matching == 0 {
            0
        } else {
            let exponent = (matching - 1) as u32;
            2_u32.pow(exponent)
        }
    }
}

impl FromStr for GameCard {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let (id, numbers) = s.split_once(':').ok_or("No colon found in string")?;
        let id = id.split_whitespace().nth(1).ok_or("Game id malformed")?;
        let id = id
            .parse::<usize>()
            .map_err(|err| format!("Could not parse id to usize: {err}"))?;
        let (winning, player) = numbers.split_once('|').ok_or("No bar found in numbers")?;
        let winning_numbers = winning
            .split_whitespace()
            .filter_map(|n| n.parse::<u32>().ok())
            .collect::<Vec<_>>();
        let player_numbers = player
            .split_whitespace()
            .filter_map(|n| n.parse::<u32>().ok())
            .collect::<Vec<_>>();
        Ok(GameCard {
            winning_numbers,
            player_numbers,
            id,
        })
    }
}

fn part_01(input: &str) -> String {
    input
        .lines()
        .map(|line| line.parse::<GameCard>().unwrap())
        .map(|gc| gc.score())
        .sum::<u32>()
        .to_string()
}

fn part_02(input: &str) -> String {
    let game_cards = input
        .lines()
        .map(|line| line.parse::<GameCard>().unwrap())
        .collect::<Vec<_>>();
    let mut copies = vec![1; game_cards.len() + 1];
    copies[0] = 0;
    for gc in game_cards.iter() {
        let id_copies = copies[gc.id];
        for id in copies.iter_mut().skip(gc.id + 1).take(gc.match_count()) {
            *id += id_copies;
        }
    }
    copies.iter().sum::<usize>().to_string()
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
        "[advent-of-code-2023:day_04:part_01] {result_01}\n\
         [advent-of-code-2023:day_04:part_02] {result_02}",
        result_01 = part_01(&input),
        result_02 = part_02(&input)
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part_01_tests {
        use super::*;
        #[test]
        fn a_game_card_with_0_matching_numbers_scores_0() {
            let game_card = GameCard {
                id: 0,
                player_numbers: vec![1, 2, 3, 4, 5],
                winning_numbers: vec![6, 7, 8, 9, 10],
            };
            assert_eq!(game_card.score(), 0);
        }

        #[test]
        fn a_game_card_with_1_matching_numbers_scores_1() {
            let game_card = GameCard {
                id: 0,
                player_numbers: vec![1, 2, 3, 4, 5],
                winning_numbers: vec![5, 6, 7, 8, 9],
            };
            assert_eq!(game_card.score(), 1);
        }

        #[test]
        fn a_game_card_with_3_matching_numbers_scores_8() {
            let game_card = GameCard {
                id: 0,
                player_numbers: vec![1, 2, 3, 4, 5],
                winning_numbers: vec![1, 6, 3, 5, 9],
            };
            assert_eq!(game_card.score(), 4);
        }

        #[test]
        fn a_game_card_string_when_parsed_has_an_id() {
            let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
            let game_card = line.parse::<GameCard>().unwrap();
            assert_eq!(game_card.id, 1);
        }

        #[test]
        fn a_game_card_string_when_multiple_spaces_before_id_can_be_parsed() {
            let line = "Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
            let game_card = line.parse::<GameCard>().unwrap();
            assert_eq!(game_card.id, 1);
        }

        #[test]
        fn a_game_card_string_when_parsed_has_winning_numbers() {
            let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
            let game_card = line.parse::<GameCard>().unwrap();
            assert_eq!(game_card.winning_numbers, [41, 48, 83, 86, 17]);
        }

        #[test]
        fn a_game_card_string_when_parsed_has_player_numbers() {
            let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
            let game_card = line.parse::<GameCard>().unwrap();
            assert_eq!(game_card.player_numbers, [83, 86, 6, 31, 17, 9, 48, 53]);
        }
    }
}

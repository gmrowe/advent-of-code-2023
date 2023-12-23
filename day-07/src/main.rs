#![allow(unused)]
use std::{env, fs, io, os::unix::thread::JoinHandleExt, str::FromStr};

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
    let mut bets = input
        .lines()
        .map(|s| s.parse::<Bet>().unwrap())
        .collect::<Vec<_>>();
    bets.sort_by(|b1, b2| b1.hand.cmp(&b2.hand));
    bets.into_iter()
        .zip(1..)
        .map(|(bet, rank)| bet.bid * rank)
        .sum::<u64>()
        .to_string()
}

fn part_02(input: &str) -> String {
    let mut bets = input
        .lines()
        .map(|s| s.parse::<Bet>().unwrap())
        .collect::<Vec<_>>();
    bets.sort_by(|b1, b2| sort_jokers_wild(&b1.hand, &b2.hand));
    bets.into_iter()
        .zip(1..)
        .map(|(bet, rank)| bet.bid * rank)
        .sum::<u64>()
        .to_string()
}

fn score_counts(counts: &[usize]) -> Score {
    fn n_of_a_kind(counts: &[usize], n: usize) -> usize {
        counts.iter().filter(|count| **count == n).count()
    }

    if n_of_a_kind(counts, 5) == 1 {
        Score::FiveOfAKind
    } else if n_of_a_kind(counts, 4) == 1 {
        Score::FourOfAKind
    } else if n_of_a_kind(counts, 3) == 1 && n_of_a_kind(counts, 2) == 1 {
        Score::FullHouse
    } else if n_of_a_kind(counts, 3) == 1 {
        Score::ThreeOfAKind
    } else if n_of_a_kind(counts, 2) == 2 {
        Score::TwoPair
    } else if n_of_a_kind(counts, 2) == 1 {
        Score::Pair
    } else {
        Score::HighCard
    }
}

fn get_jokers_wild_best_counts(hand: &Hand) -> [usize; 13] {
    let mut counts = hand.get_counts();
    let joker_index = all_cards()
        .into_iter()
        .position(|c| c == Card::Jack)
        .expect("All valid cards are in all_cards");
    let joker_count = counts[joker_index];
    counts[joker_index] -= joker_count;
    let high_count = counts.iter().max().unwrap_or(&0);
    let high_count_position = counts
        .iter()
        .position(|count| count == high_count)
        .expect("Some positon must have high count");
    counts[high_count_position] += joker_count;
    counts
}

fn sort_jokers_wild(h1: &Hand, h2: &Hand) -> std::cmp::Ordering {
    let h1_best = get_jokers_wild_best_counts(h1);
    let h2_best = get_jokers_wild_best_counts(h2);
    let h1_score = score_counts(&h1_best);
    let h2_score = score_counts(&h2_best);
    if h1_score != h2_score {
        return h1_score.cmp(&h2_score);
    }

    // If same Score type compare by cards
    h1.cards
        .iter()
        .zip(h2.cards.iter())
        .find(|(c1, c2)| c1 != c2)
        .map_or(std::cmp::Ordering::Equal, |(c1, c2)| match (c1, c2) {
            (&Card::Jack, _) => std::cmp::Ordering::Less,
            (_, &Card::Jack) => std::cmp::Ordering::Greater,
            (c1, c2) => c1.cmp(c2),
        })
}

#[derive(Eq, PartialEq, Debug, Copy, Clone, PartialOrd, Ord)]
enum Card {
    Number(u64),
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn all_cards() -> [Card; 13] {
    [
        Card::Number(2),
        Card::Number(3),
        Card::Number(4),
        Card::Number(5),
        Card::Number(6),
        Card::Number(7),
        Card::Number(8),
        Card::Number(9),
        Card::Ten,
        Card::Jack,
        Card::Queen,
        Card::King,
        Card::Ace,
    ]
}

impl TryFrom<char> for Card {
    type Error = String;

    fn try_from(n: char) -> Result<Card, <Card as TryFrom<char>>::Error> {
        match n {
            n if ('2'..='9').contains(&n) => {
                let rank = n.to_digit(10).expect("Checked that `n` is a digit");
                Ok(Card::Number(rank.into()))
            }
            'T' => Ok(Card::Ten),
            'J' => Ok(Card::Jack),
            'Q' => Ok(Card::Queen),
            'K' => Ok(Card::King),
            'A' => Ok(Card::Ace),
            other => Err(format!("`{other}` is not a valid card rank")),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Score {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
}

impl Hand {
    fn get_counts(&self) -> [usize; 13] {
        let mut counts = [0_usize; 13];
        for card in self.cards.iter() {
            let index = all_cards()
                .iter()
                .position(|c| c == card)
                .expect("All valid cands are in array");
            counts[index] += 1;
        }
        counts
    }

    fn score(&self) -> Score {
        let counts = self.get_counts();
        score_counts(&counts)
    }
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(cards_str: &str) -> Result<Hand, <Hand as std::str::FromStr>::Err> {
        let mut cards = [Card::Ace; 5];
        for (i, c) in cards_str.chars().enumerate() {
            cards[i] = Card::try_from(c)?;
        }
        Ok(Hand { cards })
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Hand) -> std::cmp::Ordering {
        let s1 = self.score();
        let s2 = other.score();
        if s1 != s2 {
            return s1.cmp(&s2);
        }
        // If same Score type compare by cards
        self.cards
            .iter()
            .zip(other.cards.iter())
            .find(|(c1, c2)| c1 != c2)
            .map_or(std::cmp::Ordering::Equal, |(c1, c2)| c1.cmp(c2))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Bet {
    hand: Hand,
    bid: u64,
}

impl FromStr for Bet {
    type Err = String;

    fn from_str(bet_str: &str) -> Result<Bet, <Bet as std::str::FromStr>::Err> {
        let (hand_str, bid_str) = bet_str
            .split_once(' ')
            .ok_or_else(|| format!("No space in bet string: `{bet_str}`"))?;
        let hand = hand_str.parse::<Hand>()?;
        let bid = bid_str
            .parse::<u64>()
            .map_err(|err| format!("Cannot parse bid: {err}"))?;
        Ok(Bet { hand, bid })
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;

    #[test]
    fn char_0_cannot_be_parsed_into_a_card() {
        assert_eq!(
            Card::try_from('0'),
            Err("`0` is not a valid card rank".into())
        )
    }

    #[test]
    fn char_1_cannot_be_parsed_into_a_card() {
        assert_eq!(
            Card::try_from('1'),
            Err("`1` is not a valid card rank".into())
        )
    }

    #[test]
    fn all_valid_num_chars_can_be_parsed_to_number_cards() -> Result<(), String> {
        let cards = "23456789TJQKA"
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<_>, _>>()?;
        assert_eq!(
            cards,
            [
                Card::Number(2),
                Card::Number(3),
                Card::Number(4),
                Card::Number(5),
                Card::Number(6),
                Card::Number(7),
                Card::Number(8),
                Card::Number(9),
                Card::Ten,
                Card::Jack,
                Card::Queen,
                Card::King,
                Card::Ace,
            ]
        );
        Ok(())
    }

    #[test]
    fn all_cards_have_proper_ordering() -> Result<(), String> {
        let cards = "23456789TJQKA"
            .chars()
            .map(Card::try_from)
            .collect::<Result<Vec<_>, _>>()?;

        assert!(cards.windows(2).all(|pair| pair[0] < pair[1]));
        Ok(())
    }

    #[test]
    fn a_string_of_cards_can_be_parsed_into_a_hand() {
        let cards = "246KA";
        let hand = Hand::from_str(cards).unwrap();
        assert_eq!(
            hand.cards,
            [
                Card::Number(2),
                Card::Number(4),
                Card::Number(6),
                Card::King,
                Card::Ace,
            ]
        )
    }

    fn assert_hand_score(hand: &str, expected_score: &Score) {
        let hand = hand.parse::<Hand>().unwrap();
        assert_eq!(&hand.score(), expected_score);
    }

    #[test]
    fn a_hand_with_a_high_card_only_can_be_scored() {
        assert_hand_score("246KA", &Score::HighCard);
    }

    #[test]
    fn a_hand_with_a_single_pair_can_be_scored() {
        assert_hand_score("242TK", &Score::Pair);
    }

    #[test]
    fn a_hand_with_two_pair_can_be_scored() {
        assert_hand_score("22AKA", &Score::TwoPair);
    }

    #[test]
    fn a_hand_with_three_of_a_kind_can_be_scored() {
        assert_hand_score("4454Q", &Score::ThreeOfAKind);
    }

    #[test]
    fn a_hand_with_a_full_house_can_be_scored() {
        assert_hand_score("777JJ", &Score::FullHouse);
    }

    #[test]
    fn a_hand_with_four_of_a_kind_can_be_scored() {
        assert_hand_score("KK2KK", &Score::FourOfAKind);
    }

    #[test]
    fn a_hand_with_five_of_a_kind_can_be_scored() {
        assert_hand_score("AAAAA", &Score::FiveOfAKind);
    }

    #[test]
    fn if_hands_scores_are_different_they_are_ordered_by_score() {
        let hands = [
            Hand::from_str("234TJ").unwrap(), // HighCard
            Hand::from_str("42TTJ").unwrap(), // Pair
            Hand::from_str("334T4").unwrap(), // TwoPair
            Hand::from_str("232T2").unwrap(), // ThreeOfAKind
            Hand::from_str("24422").unwrap(), // FullHouse
            Hand::from_str("JJJTJ").unwrap(), // FourOfAKind
            Hand::from_str("AAAAA").unwrap(), // FiveOfkind
        ];
        hands
            .windows(2)
            .for_each(|h| assert_eq!(h[0].partial_cmp(&h[1]), Some(Ordering::Less)));
    }

    #[test]
    fn if_hands_scores_are_equal_they_are_ordered_by_first_card() {
        let h1 = Hand::from_str("23456").unwrap();
        let h2 = Hand::from_str("32456").unwrap();
        assert_eq!(h1.partial_cmp(&h2), Some(Ordering::Less));
    }

    #[test]
    fn if_hands_scores_are_equal_hands_are_ordered_by_first_differing_card() {
        let h1 = Hand::from_str("22645").unwrap();
        let h2 = Hand::from_str("22546").unwrap();
        assert_eq!(h1.partial_cmp(&h2), Some(Ordering::Greater));
    }

    #[test]
    fn if_hands_have_all_same_cards_they_are_equal() {
        let h1 = Hand::from_str("23456").unwrap();
        let h2 = Hand::from_str("23456").unwrap();
        assert_eq!(h1.partial_cmp(&h2), Some(Ordering::Equal));
    }

    #[test]
    fn a_bet_has_a_hand() {
        let bet_str = "32T3K 765";
        let bet = Bet::from_str(bet_str).unwrap();
        assert_eq!(bet.hand, Hand::from_str("32T3K").unwrap());
    }

    #[test]
    fn a_bet_has_a_bid() {
        let bet_str = "32T3K 765";
        let bet = Bet::from_str(bet_str).unwrap();
        assert_eq!(bet.bid, 765);
    }

    #[test]
    fn b() {
        let h1 = Hand::from_str("KK677").unwrap();
        let h2 = Hand::from_str("KTJJT").unwrap();
        assert_eq!(sort_jokers_wild(&h1, &h2), Ordering::Less);
    }
}

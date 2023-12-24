#![allow(unused)]

use std::{collections::HashMap, env, fs, io};

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
    let (dirs, mappings) = input.split_once("\n\n").unwrap();
    let map = Map::new(dirs, mappings);
    map.path_steps_part_01().to_string()
}

fn part_02(input: &str) -> String {
    let (dirs, mappings) = input.split_once("\n\n").unwrap();
    let map = Map::new(dirs, mappings);
    map.path_steps_part_02().to_string()
}

fn gcd(x: usize, y: usize) -> usize {
    let mut a = x;
    let mut b = y;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(x: usize, y: usize) -> usize {
    (y / gcd(x, y)) * x
}

#[derive(Debug)]
struct Map {
    dirs: String,
    mappings: HashMap<String, (String, String)>,
}

impl Map {
    fn new(dirs: &str, ms: &str) -> Map {
        let mut mappings = HashMap::default();
        for line in ms.lines() {
            let (k, m) = line.split_once(" = ").unwrap();
            let (l, r) = m
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .and_then(|s| s.split_once(", "))
                .unwrap();
            mappings.insert(k.to_string(), (l.to_string(), r.to_string()));
        }
        Map {
            dirs: dirs.to_string(),
            mappings,
        }
    }

    fn path_steps_part_01(&self) -> usize {
        const START: &str = "AAA";
        const END: &str = "ZZZ";
        let mut location = START;
        for (i, c) in self.dirs.chars().cycle().enumerate() {
            if location == END {
                return i;
            }

            let mapping = self.mappings.get(location).unwrap();
            location = match c {
                'L' => &mapping.0,
                'R' => &mapping.1,
                _ => unreachable!(),
            };
        }
        0
    }

    fn path_steps_part_02(&self) -> usize {
        let cycle_length = |start: &str| -> usize {
            let mut location = start;
            for (i, c) in self.dirs.chars().cycle().enumerate() {
                if location.ends_with('Z') {
                    return i;
                }

                let mapping = self.mappings.get(location).unwrap();
                location = match c {
                    'L' => &mapping.0,
                    'R' => &mapping.1,
                    _ => unreachable!(),
                };
            }
            0
        };
        self.mappings
            .keys()
            .filter(|k| k.ends_with('A'))
            .map(|k| cycle_length(k))
            .fold(1, lcm)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod part_01 {
        use super::*;
        fn assert_path_steps(dirs: &str, mappings: &str, expected_steps: usize) {
            let map = Map::new(dirs, mappings);
            assert_eq!(map.path_steps_part_01(), expected_steps)
        }

        #[test]
        fn right_path_immediately_leads_to_dest_then_path_steps_is_1() {
            assert_path_steps("R", "AAA = (AAA, ZZZ)", 1);
        }

        #[test]
        fn left_path_immediately_leads_to_dest_then_path_steps_is_1() {
            assert_path_steps("L", "AAA = (ZZZ, AAA)", 1);
        }

        #[test]
        fn reach_dest_in_two_steps_single_mapping() {
            let map = Map::new("RL", "AAA = (ZZZ, AAA)");
        }

        #[test]
        fn reach_dest_before_end_of_dirs() {
            assert_path_steps("RLLLR", "AAA = (ZZZ, AAA)", 2);
        }

        #[test]
        fn reach_dest_in_two_steps_two_mappings() {
            let dirs = "LL";
            let mappings = "AAA = (BBB, CCC)\n\
                        BBB = (ZZZ, EEE)";
            assert_path_steps(dirs, mappings, 2);
        }

        #[test]
        fn reach_dest_in_two_steps_must_repeat_dirs() {
            let dirs = "L";
            let mappings = "AAA = (BBB, CCC)\n\
                            BBB = (ZZZ, EEE)";
            assert_path_steps(dirs, mappings, 2);
        }

        #[test]
        fn reach_dest_in_example_map() {
            let dirs = "LLR";
            let mappings = "AAA = (BBB, BBB)\n\
                            BBB = (AAA, ZZZ)\n\
                            ZZZ = (ZZZ, ZZZ)";
            assert_path_steps(dirs, mappings, 6);
        }
    }

    mod part_02 {
        use super::*;

        #[test]
        fn f() {
            let dirs = "LR";
            let mappings = "11A = (11B, XXX)\n\
                            11B = (XXX, 11Z)\n\
                            11Z = (11B, XXX)\n\
                            22A = (22B, XXX)\n\
                            22B = (22C, 22C)\n\
                            22C = (22Z, 22Z)\n\
                            22Z = (22B, 22B)\n\
                            XXX = (XXX, XXX)";
            let map = Map::new(dirs, mappings);
            assert_eq!(map.path_steps_part_02(), 6);
        }
    }
}

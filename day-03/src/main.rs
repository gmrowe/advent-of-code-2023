use std::{env, fs, io, str::FromStr};

#[derive(Clone, Copy, Debug)]
enum Elem {
    Empty,
    Num(u32),
    Sym(char),
}

#[derive(Clone, Copy, Debug)]
struct MapNum {
    value: u32,
    start_index: usize,
    length: usize,
}

#[derive(Clone, Debug)]
struct Map {
    symbols: Vec<Elem>,
    stride: usize,
}

impl Map {
    fn map_nums(&self) -> Vec<MapNum> {
        let mut nums = Vec::new();
        let mut parsing_num = false;
        let mut num = 0;
        let mut start_index = 0;
        for (index, elem) in self.symbols.iter().enumerate() {
            // If we reach the end of the line when parsing a number
            // Push the number and reset all flags
            if index % self.stride == 0 && parsing_num {
                nums.push(MapNum {
                    value: num,
                    start_index,
                    length: index - start_index,
                });
                parsing_num = false;
                num = 0;
            }

            match elem {
                Elem::Empty | Elem::Sym(_) => {
                    if parsing_num {
                        nums.push(MapNum {
                            value: num,
                            start_index,
                            length: index - start_index,
                        });
                        parsing_num = false;
                        num = 0;
                    }
                }

                Elem::Num(n) => {
                    if !parsing_num {
                        start_index = index;
                    }
                    parsing_num = true;
                    num = 10 * num + n;
                }
            }
        }
        if parsing_num {
            nums.push(MapNum {
                value: num,
                start_index,
                length: self.symbols.len() - start_index,
            });
        }
        nums
    }
    fn part_numbers(&self) -> Vec<u32> {
        let has_symbol_neighbor = |i| {
            self.neighbor_indices(i)
                .into_iter()
                .any(|n| matches!(self.symbols[n], Elem::Sym(_)))
        };
        self.map_nums()
            .iter()
            .filter(|n| (n.start_index..n.start_index + n.length).any(has_symbol_neighbor))
            .map(|n| n.value)
            .collect()
    }

    fn neighbor_indices(&self, index: usize) -> Vec<usize> {
        let num_rows = self.symbols.len() / self.stride;
        let row = index / self.stride;
        let col = index % self.stride;

        let mut indices = Vec::new();
        for r in -1..=1 {
            for c in -1..=1 {
                if let (Some(new_row), Some(new_col)) =
                    (row.checked_add_signed(r), col.checked_add_signed(c))
                {
                    if (r != 0 || c != 0) && new_row < num_rows && new_col < self.stride {
                        indices.push(new_col + new_row * self.stride);
                    }
                }
            }
        }
        indices
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, <Self as FromStr>::Err> {
        const RADIX: u32 = 10;
        let mut symbols = Vec::new();
        let line_len = input.lines().next().map(|s| s.len()).unwrap_or(0);
        for line in input.lines() {
            if line.len() != line_len {
                return Err("Not all lines the same length");
            }

            for ch in line.chars() {
                let elem = match ch {
                    '.' => Elem::Empty,
                    d if d.is_ascii_digit() => {
                        let digit = d.to_digit(RADIX).expect("d is a digit");
                        Elem::Num(digit)
                    }
                    other => Elem::Sym(other),
                };
                symbols.push(elem);
            }
        }
        let stride = symbols.len() / input.lines().count();
        Ok(Map { symbols, stride })
    }
}

fn part_01(input: &str) -> String {
    let map = input.parse::<Map>().expect("Input is well formed");
    map.part_numbers().iter().sum::<u32>().to_string()
}

fn part_02(input: &str) -> String {
    "TODO".to_string()
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
        "[advent-of-code-2023:day_03:part_01] {result_01}\n\
         [advent-of-code-2023:day_03:part_02] {result_02}",
        result_01 = part_01(&input),
        result_02 = part_02(&input)
    );
    Ok(())
}

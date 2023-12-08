use std::str::FromStr;

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
    fn numeric_neighbors(&self, index: usize) -> Vec<u32> {
        self.neighbor_indices(index)
            .into_iter()
            .filter_map(|i| match self.symbols[i] {
                Elem::Num(n) => Some(n),
                _ => None,
            })
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

fn main() {
    // [X] parse input into Map type
    // [ ] given and index, list all numbers adjacent
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_stride_is_length_of_lines() {
        let i = "...\n\
                 .#.\n\
                 ...";
        let m = i.parse::<Map>().unwrap();
        assert_eq!(m.stride, 3);
    }

    #[test]
    fn index_with_no_numeric_neighbors() {
        let i = "...\n\
                 .#.\n\
                 ...";
        let m = i.parse::<Map>().unwrap();
        let index = 4; // Index of '#'
        assert_eq!(m.numeric_neighbors(index), []);
    }

    #[test]
    fn index_with_top_left_numeric_neighbors() {
        let i = "3..\n\
                 .#.\n\
                 ...";
        let m = i.parse::<Map>().unwrap();
        let index = 4; // Index of '#'
        assert_eq!(m.numeric_neighbors(index), [3]);
    }

    #[test]
    fn index_with_multiple_neighbors() {
        let i = "3.4\n\
                 5#6\n\
                 .7.";
        let m = i.parse::<Map>().unwrap();
        let index = 4; // Index of '#'
        assert_eq!(m.numeric_neighbors(index), [3, 4, 5, 6, 7]);
    }

    #[test]
    fn index_with_multi_digit_neighbor() {
        let i = "345\n\
                 .#.\n\
                 ...";
        let m = i.parse::<Map>().unwrap();
        let index = 4; // Index of '#'
        assert_eq!(m.numeric_neighbors(index), [345]);
    }
}

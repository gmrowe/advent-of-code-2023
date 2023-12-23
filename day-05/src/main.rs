use std::{collections::HashMap, env, fs, io, str::FromStr};

fn part_01(input: &str) -> String {
    let (seeds, ag_data) = input.split_once("\n\n").unwrap();
    let seeds = seeds
        .strip_prefix("seeds: ")
        .and_then(|ss| parse_nums(ss).ok())
        .unwrap();
    let ag_map = ag_data.parse::<AgMap>().unwrap();
    seeds
        .into_iter()
        .filter_map(|s| ag_map.convert("seed", s, "location"))
        .min()
        .map(|n| n.to_string())
        .unwrap_or("No locations found".to_string())
}

fn part_02(input: &str) -> String {
    let (seeds, ag_data) = input.split_once("\n\n").unwrap();
    let seeds = seeds
        .strip_prefix("seeds: ")
        .and_then(|ss| parse_nums(ss).ok())
        .unwrap();
    let ag_map = ag_data.parse::<AgMap>().unwrap();

    seeds
        .chunks_exact(2)
        .flat_map(|c| c[0]..c[0] + c[1])
        .filter_map(|s| ag_map.convert("seed", s, "location"))
        .min()
        .map(|n| n.to_string())
        .unwrap_or("No locations found".to_string())
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

fn parse_nums(nums: &str) -> Result<Vec<u64>, String> {
    nums.split_whitespace()
        .map(|n| n.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|err| format!("Could not parse nums: {err}"))
}

#[derive(Debug, Clone, Copy)]
pub struct MappingRange {
    source_range_start: u64,
    dest_range_start: u64,
    range_length: u64,
}

impl MappingRange {
    pub fn new(dest_range_start: u64, source_range_start: u64, range_length: u64) -> MappingRange {
        MappingRange {
            dest_range_start,
            source_range_start,
            range_length,
        }
    }

    pub fn is_in_range(&self, source: u64) -> bool {
        self.source_range_start <= source && source < self.source_range_start + self.range_length
    }

    pub fn calc_dest(&self, source: u64) -> u64 {
        let delta = source - self.source_range_start;
        self.dest_range_start + delta
    }
}

impl FromStr for MappingRange {
    type Err = String;
    fn from_str(input: &str) -> Result<MappingRange, String> {
        parse_nums(input).map(|vec| {
            Ok(MappingRange::new(
                *vec.get(0).ok_or("No first number")?,
                *vec.get(1).ok_or("No second number")?,
                *vec.get(2).ok_or("No third number")?,
            ))
        })?
    }
}

#[derive(Debug, Clone, Default)]
pub struct Mapping {
    ranges: Vec<MappingRange>,
    source_category: String,
    dest_category: String,
}

impl Mapping {
    pub fn new(source_category: &str, dest_category: &str) -> Self {
        Mapping {
            source_category: source_category.to_string(),
            dest_category: dest_category.to_string(),
            ..Self::default()
        }
    }
    pub fn with_range(mut self, mapping_range: MappingRange) -> Self {
        self.ranges.push(mapping_range);
        self
    }

    pub fn convert(&self, source: u64) -> u64 {
        self.ranges
            .iter()
            .find(|r| r.is_in_range(source))
            .map(|r| r.calc_dest(source))
            .unwrap_or(source)
    }
}

impl FromStr for Mapping {
    type Err = String;
    fn from_str(mapping: &str) -> Result<Self, Self::Err> {
        let mut mapping_lines = mapping.lines();
        let (source, dest) = mapping_lines
            .next()
            .and_then(|line| line.strip_suffix(" map:"))
            .and_then(|prefix| prefix.split_once("-to-"))
            .ok_or_else(|| format!("Mapping format error: {mapping}"))?;

        let mappings = mapping_lines
            .map(|line| line.parse::<MappingRange>())
            .collect::<Result<Vec<_>, _>>()?;

        let new_mapping = mappings
            .into_iter()
            .fold(Mapping::new(source, dest), |m, rng| m.with_range(rng));
        Ok(new_mapping)
    }
}

#[derive(Default, Clone, Debug)]
pub struct AgMap {
    mappings: HashMap<String, Mapping>,
}

impl AgMap {
    pub fn mappings_len(&self) -> usize {
        self.mappings.len()
    }

    pub fn with_mapping(mut self, mapping: Mapping) -> AgMap {
        self.mappings
            .insert(mapping.source_category.clone(), mapping);
        self
    }

    pub fn convert(
        &self,
        source_category: &str,
        source_id: u64,
        dest_category: &str,
    ) -> Option<u64> {
        let mut category = source_category;
        let mut id = source_id;
        while category != dest_category {
            let mapping = self.mappings.get(category)?;
            id = mapping.convert(id);
            category = &mapping.dest_category;
        }
        Some(id)
    }
}

impl FromStr for AgMap {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mappings = s
            .split("\n\n")
            .map(|mapping| mapping.parse::<Mapping>())
            .collect::<Result<Vec<_>, _>>()?;

        let ag_map = mappings
            .into_iter()
            .fold(AgMap::default(), |ag_map, m| ag_map.with_mapping(m));
        Ok(ag_map)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod mapping_range {
        use super::*;

        fn test_mapping_range() -> MappingRange {
            MappingRange::new(98, 52, 4)
        }

        #[test]
        fn when_input_is_in_source_range_in_range_is_true() {
            let seed_number = 55;
            let seed_to_soil_map = test_mapping_range();
            assert!(seed_to_soil_map.is_in_range(seed_number));
        }

        #[test]
        fn when_input_is_smaller_than_source_range_start_in_range_is_false() {
            let seed_number = 51;
            let seed_to_soil_map = test_mapping_range();
            assert!(!seed_to_soil_map.is_in_range(seed_number));
        }

        #[test]
        fn when_input_is_larger_than_source_range_start_in_range_is_false() {
            let seed_number = 56;
            let seed_to_soil_map = test_mapping_range();
            assert!(!seed_to_soil_map.is_in_range(seed_number));
        }

        #[test]
        fn when_given_a_source_num_it_can_calculate_a_destination_num() {
            let seed_number = 53;
            let seed_to_soil_map = test_mapping_range();
            assert_eq!(seed_to_soil_map.calc_dest(seed_number), 99)
        }
    }

    mod mapping {
        use super::*;

        #[test]
        fn when_source_is_in_a_range_the_range_converts_it() {
            let mapping = Mapping::default()
                .with_range(MappingRange::new(50, 98, 2))
                .with_range(MappingRange::new(52, 50, 48));
            let source = 55;
            assert_eq!(mapping.convert(source), 57);
        }

        #[test]
        fn when_source_is_not_in_a_range_it_is_converted_to_same_dest_number() {
            let mapping = Mapping::default()
                .with_range(MappingRange::new(50, 98, 2))
                .with_range(MappingRange::new(52, 50, 48));
            let source = 10;
            assert_eq!(mapping.convert(source), 10);
        }

        #[test]
        fn when_quried_returns_its_source_category() {
            let mapping = Mapping::new("seed", "");
            assert_eq!(mapping.source_category, "seed")
        }

        #[test]
        fn when_quried_returns_its_destination_category() {
            let mapping = Mapping::new("", "fertilizer");
            assert_eq!(mapping.dest_category, "fertilizer")
        }

        #[test]
        fn can_be_parsed_from_a_string() {
            let text = "seed-to-soil map:\n\
                        50 98 2\n\
                        52 50 48";
            let mapping = text.parse::<Mapping>().unwrap();
            assert_eq!(mapping.convert(55), 57);
        }
    }
    mod agmap {
        use super::*;

        #[test]
        fn when_first_created_it_has_no_mappings() {
            let ag_map = AgMap::default();
            assert_eq!(ag_map.mappings_len(), 0)
        }

        #[test]
        fn can_have_mappings_added() {
            let ag_map = AgMap::default().with_mapping(Mapping::default());
            assert_eq!(ag_map.mappings_len(), 1);
        }

        #[test]
        fn convert_source_input_to_direct_dest_output() {
            let seed_to_soil = Mapping::new("seed", "soil")
                .with_range(MappingRange::new(50, 98, 2))
                .with_range(MappingRange::new(52, 50, 48));
            let ag_map = AgMap::default().with_mapping(seed_to_soil);
            assert_eq!(ag_map.convert("seed", 14, "soil"), Some(14));
        }

        #[test]
        fn convert_source_input_to_indirect_dest_output() {
            let seed_to_soil = Mapping::new("seed", "soil")
                .with_range(MappingRange::new(50, 98, 2))
                .with_range(MappingRange::new(52, 50, 48));
            let soil_to_fertilizer = Mapping::new("soil", "fertilizer")
                .with_range(MappingRange::new(0, 15, 37))
                .with_range(MappingRange::new(37, 52, 2))
                .with_range(MappingRange::new(39, 0, 15));
            let ag_map = AgMap::default()
                .with_mapping(seed_to_soil)
                .with_mapping(soil_to_fertilizer);
            assert_eq!(ag_map.convert("seed", 14, "fertilizer"), Some(53));
        }

        #[test]
        fn returns_none_if_source_input_cannot_convert_to_dest_output() {
            let seed_to_soil = Mapping::new("seed", "soil")
                .with_range(MappingRange::new(50, 98, 2))
                .with_range(MappingRange::new(52, 50, 48));
            let soil_to_fertilizer = Mapping::new("soil", "fertilizer")
                .with_range(MappingRange::new(0, 15, 37))
                .with_range(MappingRange::new(37, 52, 2))
                .with_range(MappingRange::new(39, 0, 15));
            let ag_map = AgMap::default()
                .with_mapping(seed_to_soil)
                .with_mapping(soil_to_fertilizer);
            assert_eq!(ag_map.convert("seed", 14, "farmland"), None);
        }

        #[test]
        fn an_ag_map_can_be_parsed_from_a_string() {
            let text = "seed-to-soil map:\n\
                        50 98 2\n\
                        52 50 48\n\
                        \n\
                        soil-to-fertilizer map:\n\
                        0 15 37\n\
                        37 52 2\n\
                        39 0 15\n\
                        \n\
                        fertilizer-to-water map:\n\
                        49 53 8\n\
                        0 11 42\n\
                        42 0 7\n\
                        57 7 4";
            let ag_map = text.parse::<AgMap>().unwrap();
            assert_eq!(ag_map.convert("seed", 55, "water"), Some(53));
        }
    }
}

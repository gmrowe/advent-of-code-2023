//

fn main() {
    println!("Hello, world!");
}

pub struct MappingRange {
    source_range_start: u32,
    dest_range_start: u32,
    range_length: u32,
}

impl MappingRange {
    pub fn new(source_range_start: u32, dest_range_start: u32, range_length: u32) -> MappingRange {
        MappingRange {
            source_range_start,
            dest_range_start,
            range_length,
        }
    }
    pub fn source_in_range(&self, source: u32) -> bool {
        self.source_range_start <= source && source < self.source_range_start + self.range_length
    }

    pub fn calc_dest(&self, source: u32) -> u32 {
        let delta = source - self.source_range_start;
        self.dest_range_start + delta
    }
}

pub struct Mapping {
    source_cat: &'static str,
    dest_cat: &'static str,
    ranges: Vec<MappingRange>,
}

impl Mapping {
    pub fn convert(&self, source: u32) -> u32 {
        self.ranges
            .iter()
            .find(|r| r.source_in_range(source))
            .map(|r| r.calc_dest(source))
            .unwrap_or(source)
    }
}

struct AgMap {}

impl AgMap {
    pub fn new() -> Self {
        AgMap {}
    }

    pub fn with_mapping(self, mapping: Mapping) -> Self {
        todo!()
    }

    pub fn convert(&self, source: &str, dest: &str, source_num: u32) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    mod with_a_valid_mapping_range {
        use super::*;

        fn test_mapping_range() -> MappingRange {
            MappingRange::new(52, 98, 4)
        }

        #[test]
        fn when_input_is_in_source_range_in_range_is_true() {
            let seed_number = 55;
            let seed_to_soil_map = test_mapping_range();
            assert!(seed_to_soil_map.source_in_range(seed_number));
        }

        #[test]
        fn when_input_is_smaller_than_source_range_start_in_range_is_false() {
            let seed_number = 51;
            let seed_to_soil_map = test_mapping_range();
            assert!(!seed_to_soil_map.source_in_range(seed_number));
        }

        #[test]
        fn when_input_is_larger_than_source_range_start_in_range_is_false() {
            let seed_number = 56;
            let seed_to_soil_map = test_mapping_range();
            assert!(!seed_to_soil_map.source_in_range(seed_number));
        }

        #[test]
        fn when_given_a_source_num_it_can_calculate_a_destination_num() {
            let seed_number = 53;
            let seed_to_soil_map = test_mapping_range();
            assert_eq!(seed_to_soil_map.calc_dest(seed_number), 99)
        }
    }

    #[test]
    fn when_source_is_in_a_range_the_range_converts_it() {
        let mapping = Mapping {
            source_cat: "",
            dest_cat: "",
            ranges: vec![MappingRange::new(50, 98, 2), MappingRange::new(52, 50, 48)],
        };
        let source = 55;
        assert_eq!(mapping.convert(source), 53);
    }

    #[test]
    fn when_source_is_not_in_a_range_it_is_converted_to_same_dest_number() {
        let mapping = Mapping {
            source_cat: "",
            dest_cat: "",
            ranges: vec![MappingRange::new(50, 98, 2), MappingRange::new(52, 50, 48)],
        };
        let source = 10;
        assert_eq!(mapping.convert(source), 10);
    }

    #[test]
    fn f() {
        let seed_to_soil = Mapping {
            source_cat: "seed",
            dest_cat: "soil",
            ranges: vec![MappingRange::new(50, 98, 2), MappingRange::new(52, 50, 48)],
        };

        let soil_to_fertilizer = Mapping {
            source_cat: "soil",
            dest_cat: "fertilizer",
            ranges: vec![
                MappingRange::new(0, 15, 37),
                MappingRange::new(37, 52, 2),
                MappingRange::new(39, 0, 15),
            ],
        };
        let ag_map = AgMap::new()
            .with_mapping(seed_to_soil)
            .with_mapping(soil_to_fertilizer);
        assert_eq!(ag_map.convert("seed", "fertilizer", 13), 52)
    }
}

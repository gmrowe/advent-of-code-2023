use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Clone, Copy)]
pub struct MappingRange {
    source_range_start: u32,
    dest_range_start: u32,
    range_length: u32,
}

impl MappingRange {
    pub fn new(dest_range_start: u32, source_range_start: u32, range_length: u32) -> MappingRange {
        MappingRange {
            dest_range_start,
            source_range_start,
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

    pub fn convert(&self, source: u32) -> u32 {
        self.ranges
            .iter()
            .find(|r| r.source_in_range(source))
            .map(|r| r.calc_dest(source))
            .unwrap_or(source)
    }
}

#[derive(Default, Clone)]
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
        dest_category: &str,
        source_id: u32,
    ) -> Option<u32> {
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
            assert_eq!(ag_map.convert("seed", "soil", 14), Some(14));
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
            assert_eq!(ag_map.convert("seed", "fertilizer", 14), Some(53));
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
            assert_eq!(ag_map.convert("seed", "farmland", 14), None);
        }
    }
}

//

fn main() {
    println!("Hello, world!");
}

struct Mapping {
    source_range_start: u32,
    range_length: u32,
}

impl Mapping {
    fn in_source_range(&self, source: u32) -> bool {
        self.source_range_start <= source && source < self.source_range_start + self.range_length
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn when_a_seed_number_is_in_source_range_in_range_is_true() {
        let seed_number = 55;
        let seed_to_soil_map = Mapping {
            source_range_start: 52,
            range_length: 4,
        };
        assert!(seed_to_soil_map.in_source_range(seed_number));
    }

    #[test]
    fn when_a_seed_number_is_smaller_than_source_range_start_in_range_is_false() {
        let seed_number = 51;
        let seed_to_soil_map = Mapping {
            source_range_start: 52,
            range_length: 4,
        };
        assert!(!seed_to_soil_map.in_source_range(seed_number));
    }

    #[test]
    fn when_a_seed_number_is_larger_than_source_range_start_in_range_is_false() {
        let seed_number = 56;
        let seed_to_soil_map = Mapping {
            source_range_start: 52,
            range_length: 4,
        };
        assert!(!seed_to_soil_map.in_source_range(seed_number));
    }
}

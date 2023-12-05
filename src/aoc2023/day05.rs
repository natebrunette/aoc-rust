// https://adventofcode.com/2023/day/5

use itertools::Itertools;
use std::str::FromStr;

pub fn part1(input: String) -> u64 {
    let almanac: Almanac = input.parse().unwrap();

    almanac.find_closest_location()
}

pub fn part2(input: String) -> u64 {
    let mut almanac: Almanac = input.parse().unwrap();

    almanac.expand_seeds();
    almanac.find_closest_location()
}

struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Map,
    soil_to_fertilizer: Map,
    fertilizer_to_water: Map,
    water_to_light: Map,
    light_to_temperature: Map,
    temperature_to_humidity: Map,
    humidity_to_location: Map,
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let seeds: Vec<_> = input
            .lines()
            .skip_while(|&line| !line.contains("seeds"))
            .take(1)
            .collect::<String>()
            .split_whitespace()
            .skip(1)
            .map(|num| num.trim().parse::<u64>().unwrap())
            .collect();
        let seed_to_soil = create_map(input, "seed-to-soil map:");
        let soil_to_fertilizer = create_map(input, "soil-to-fertilizer map:");
        let fertilizer_to_water = create_map(input, "fertilizer-to-water map:");
        let water_to_light = create_map(input, "water-to-light map:");
        let light_to_temperature = create_map(input, "light-to-temperature map:");
        let temperature_to_humidity = create_map(input, "temperature-to-humidity map:");
        let humidity_to_location = create_map(input, "humidity-to-location map:");

        Ok(Almanac {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

impl Almanac {
    fn expand_seeds(&mut self) {
        self.seeds = self
            .seeds
            .chunks_exact(2)
            .flat_map(|range| (range[0]..(range[0] + range[1])).collect::<Vec<u64>>())
            .unique()
            .collect();
    }

    fn find_closest_location(&self) -> u64 {
        self.seeds
            .iter()
            .map(|&seed| self.get_seed_to_soil(seed))
            .map(|soil| self.get_soil_to_fertilizer(soil))
            .map(|fertilizer| self.get_fertilizer_to_water(fertilizer))
            .map(|water| self.get_water_to_light(water))
            .map(|light| self.get_light_to_temperature(light))
            .map(|temperature| self.get_temperature_to_humidity(temperature))
            .map(|humidity| self.get_humidity_to_location(humidity))
            .min()
            .unwrap()
            .clone()
    }

    fn get_seed_to_soil(&self, seed: u64) -> u64 {
        self.seed_to_soil.get(seed)
    }

    fn get_soil_to_fertilizer(&self, soil: u64) -> u64 {
        self.soil_to_fertilizer.get(soil)
    }

    fn get_fertilizer_to_water(&self, fertilizer: u64) -> u64 {
        self.fertilizer_to_water.get(fertilizer)
    }

    fn get_water_to_light(&self, water: u64) -> u64 {
        self.water_to_light.get(water)
    }

    fn get_light_to_temperature(&self, light: u64) -> u64 {
        self.light_to_temperature.get(light)
    }

    fn get_temperature_to_humidity(&self, temperature: u64) -> u64 {
        self.temperature_to_humidity.get(temperature)
    }

    fn get_humidity_to_location(&self, humidity: u64) -> u64 {
        self.humidity_to_location.get(humidity)
    }
}

struct Map {
    elements: Vec<MapEntry>,
}

impl FromIterator<MapEntry> for Map {
    fn from_iter<T: IntoIterator<Item = MapEntry>>(iter: T) -> Self {
        Map {
            elements: iter.into_iter().collect(),
        }
    }
}

impl Map {
    fn get(&self, key: u64) -> u64 {
        for element in self.elements.iter() {
            if element.contains(key) {
                let offset = key - element.source;
                return element.dest + offset;
            }
        }

        key
    }
}

struct MapEntry {
    source: u64,
    dest: u64,
    increment: u64,
}

impl From<(u64, u64, u64)> for MapEntry {
    fn from(value: (u64, u64, u64)) -> Self {
        MapEntry {
            source: value.0,
            dest: value.1,
            increment: value.2,
        }
    }
}

impl MapEntry {
    fn contains(&self, num: u64) -> bool {
        num >= self.source && num < self.source + self.increment
    }
}

fn create_map(input: &str, name: &str) -> Map {
    input
        .lines()
        .skip_while(|&line| line != name)
        .skip(1)
        .take_while(|&line| !line.is_empty())
        .collect::<Vec<_>>()
        .iter()
        .map(|&line| {
            let args: Vec<_> = line
                .split_whitespace()
                .map(|str| str.parse::<u64>().unwrap())
                .collect();
            MapEntry::from((args[1], args[0], args[2]))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::get_input;

    #[test]
    fn part1_sample_test() {
        let input = get_input("aoc2023/res/day05_sample.txt");
        assert_eq!(part1(input), 35);
    }

    #[test]
    fn part1_test() {
        let input = get_input("aoc2023/res/day05.txt");
        assert_eq!(part1(input), 324724204);
    }

    #[test]
    fn part2_sample_test() {
        let input = get_input("aoc2023/res/day05_sample.txt");
        assert_eq!(part2(input), 46);
    }

    #[test]
    fn part2_test() {
        let input = get_input("aoc2023/res/day05.txt");
        assert_eq!(part2(input), 104070862);
    }
}

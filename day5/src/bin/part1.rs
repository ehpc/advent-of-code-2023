use std::collections::{BTreeSet};
use std::fs::File;
use std::io;
use std::io::{BufRead, Error};

struct SmartMap {
    sources: Vec<u64>,
    destinations: Vec<u64>,
    counts: Vec<u64>,
}

impl SmartMap {
    fn new() -> Self {
        SmartMap { sources: vec![], destinations: vec![], counts: vec![] }
    }

    fn add_range(&mut self, source: u64, destination: u64, count: u64) {
        self.sources.push(source);
        self.destinations.push(destination);
        self.counts.push(count);
    }

    fn source_to_destination(&self, source: u64) -> u64 {
        let index = self.sources.iter().enumerate().position(|(index, &current_source)| {
            source >= current_source && source < current_source + self.counts[index]
        });
        if index.is_none() {
            return source;
        }
        let source_shift = source - self.sources[index.unwrap()];
        return self.destinations[index.unwrap()] + source_shift;
    }
}

fn main() -> Result<(), Error> {
    let lines = get_lines();
    let seeds = get_seeds(&lines);
    let maps = get_maps(&lines);
    let locations = get_locations(&seeds, &maps);
    let min_location = locations.iter().next().unwrap();
    println!("{:?}", min_location);
    Ok(())
}

fn get_locations(seeds: &Vec<u64>, maps: &Vec<SmartMap>) -> BTreeSet<u64> {
    seeds.iter().fold(BTreeSet::new(), |mut acc, seed| {
        let location = maps.iter().fold(*seed, |source, map| {
            return map.source_to_destination(source);
        });
        acc.insert(location);
        acc
    })
}

fn get_maps(lines: &Vec<String>) -> Vec<SmartMap> {
    let mut maps: Vec<SmartMap> = vec![];
    lines[1..].iter().for_each(|line| {
        if line.chars().count() == 0 {
            maps.push(SmartMap::new());
            return
        }
        if line.chars().next().unwrap().is_numeric() {
            let range_parts: Vec<u64> = line.split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
            let destination = range_parts[0];
            let source = range_parts[1];
            let count = range_parts[2];
            let map = maps.last_mut().unwrap();
            map.add_range(source, destination, count);
        }
    });
    maps
}

fn get_lines() -> Vec<String> {
    let file = File::open("input1.txt").unwrap();
    io::BufReader::new(file).lines().map(|line| line.unwrap()).collect()
}

fn get_seeds(lines: &Vec<String>) -> Vec<u64> {
    lines[0].split(':')
        .next_back()
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}
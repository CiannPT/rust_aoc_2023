advent_of_code::solution!(5);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut seeds:Vec<u64> = Vec::new();
    for section in input.split("\n\n").filter(|&x| !x.is_empty()) {
        if section.starts_with("seeds:"){
            seeds = section.split(':').collect::<Vec<&str>>()[1]
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            continue;
        }
        let section_map = read_section_map(section);
        for seed in 0..seeds.len() {
            for transform in &section_map {
                if seeds[seed]>=transform.1 && seeds[seed]<=transform.1+transform.2{
                    seeds[seed]=seeds[seed]+transform.0-transform.1;
                    break;
                }
            }
        }
    }
    Some(seeds.iter().min().unwrap().to_owned().try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    // let mut seeds:Vec<u64> = Vec::new();
    let (seeds_map, section_maps) = match input.split("\n\n").filter(|&x| !x.is_empty()).collect::<Vec<&str>>().split_first() {
        Some(s) => (read_seeds_line_part2(s.0.to_owned()),
            s.1.into_iter().map(|sec| read_section_map(sec)).collect::<Vec<Vec<(u64,u64,u64)>>>()),
        None => panic!("bad split sections"),
    };
    let mut min_value = u64::max_value();
    for seed_batch in seeds_map {
        for mut seed in seed_batch.0..(seed_batch.0+seed_batch.1) {
            for section in &section_maps {
                for transform in section {
                    if seed>=transform.1 && seed<=transform.1+transform.2{
                        seed=seed+transform.0-transform.1;
                        break;
                    }
                }
            }
            if seed < min_value {
                min_value = seed;
            }
        }
    }
    Some(min_value.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}


fn read_section_map(section:&str) -> Vec<(u64, u64, u64)> {
    let mut map:Vec<(u64,u64,u64)> = Vec::new();
    for line in section.split("\n").filter(|&x| !x.is_empty()) {
        if line.ends_with(':') {
            continue;
        }
        map.push(line.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect_tuple().unwrap());
    }
    return map;
}

fn read_seeds_line_part2(line:&str) -> Vec<(u64, u64)> {
    let mut seeds:Vec<(u64,u64)> = Vec::new();
    let mut seeds_map = line.split(':').collect::<Vec<&str>>()[1]
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    while seeds_map.len()>0 {
        let range = seeds_map.pop().unwrap();
        let start = seeds_map.pop().unwrap();
        seeds.push((start,range));
    };
    return seeds;
}

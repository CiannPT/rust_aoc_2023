use std::collections::HashMap;
use regex::Regex;
use num::integer::lcm;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (lr_line, map_lines) = input.split_once("\n").unwrap();
    let mut map:HashMap<&str,(&str,&str)> = HashMap::new();
    let reg: Regex = Regex::new(r"(?<Key>\w{3}) = \((?<L>\w{3}), (?<R>\w{3})\)").unwrap();
    for line in map_lines.split("\n").filter(|&x| !x.is_empty()) {
        if let Some(c) = reg.captures(line) {
            map.insert(c.name("Key").unwrap().as_str(), (c.name("L").unwrap().as_str(),c.name("R").unwrap().as_str()));
        }
    }
    let mut current_pos = "AAA";
    let mut count =0;
    while current_pos!= "ZZZ" && count <100_000 {
        for step in lr_line.chars() {
            current_pos = match step {
                'L' => map[current_pos].0,
                'R' => map[current_pos].1,
                _ => panic!("Not L or R")
            };
            count +=1;
            if current_pos == "ZZZ" {
                break;
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (lr_line, map_lines) = input.split_once("\n").unwrap();
    let mut map:HashMap<&str,(&str,&str)> = HashMap::new();
    let reg: Regex = Regex::new(r"(?<Key>\w{3}) = \((?<L>\w{3}), (?<R>\w{3})\)").unwrap();
    let mut pos_vec:Vec<&str> = vec![];
    for line in map_lines.split("\n").filter(|&x| !x.is_empty()) {
        if let Some(c) = reg.captures(line) {
            map.insert(c.name("Key").unwrap().as_str(), (c.name("L").unwrap().as_str(),c.name("R").unwrap().as_str()));
            if c.name("Key").unwrap().as_str().ends_with("A") {
                pos_vec.push(c.name("Key").unwrap().as_str());
            }
        }
    }
    // println!("{:?}",pos_vec);
    let mut count_lcm:u64 =1;
    for current_pos in 0..pos_vec.len() {
        let mut count =0;
        while !pos_vec[current_pos].ends_with("Z") && count < 100_000_000 {
            for step in lr_line.chars() {
                pos_vec[current_pos] = match step {
                    'L' => map[pos_vec[current_pos]].0,
                    'R' => map[pos_vec[current_pos]].1,
                    _ => panic!("Not L or R")
                };
                count +=1;
                if pos_vec[current_pos].ends_with("Z") {
                    break;
                }
            }
        }
        count_lcm = lcm(count_lcm, count);
        // println!("{:?} - {}",pos_vec, count);
    }
    Some(count_lcm)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

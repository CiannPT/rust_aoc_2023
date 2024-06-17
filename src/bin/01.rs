advent_of_code::solution!(1);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    let re:Regex = Regex::new(r"\d").unwrap();
    for mut line in input.split("\n") {
        line = line.trim();
        let re_matches: Vec<&str> = re.find_iter(line).map(|m| m.as_str()).collect();
        let first_val1 = re_matches.first().unwrap_or(&"0").parse::<u32>().unwrap();
        let last_val1 = re_matches.last().unwrap_or(&"0").parse::<u32>().unwrap();
        sum += first_val1*10+last_val1;
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum2: u32 = 0;
    let re:Regex = Regex::new(r"\d|one|two|three|four|five|six|seven|eight|nine").unwrap();
    for mut line in input.split("\n") {
        line = line.trim();
        let re_matches2: Vec<&str> = re.find_iter_overlapping(line).map(|m| m.as_str()).collect();
        let first_val2 = parse_to_int(re_matches2.first().unwrap_or(&"0"));
        let last_val2 = parse_to_int(re_matches2.last().unwrap_or(&"0"));
        sum2 += first_val2*10+last_val2;
    }
    Some(sum2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}


fn parse_to_int(input: &str) -> u32 {
    return match input {
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    };
}

trait RegexExt {
    fn find_iter_overlapping<'r, 'h>(&'r self, haystack: &'h str) -> MatchesOverlapping<'r, 'h>;
}

impl RegexExt for regex::Regex {
    fn find_iter_overlapping<'r, 'h>(&'r self, haystack: &'h str) -> MatchesOverlapping<'r, 'h>{
        MatchesOverlapping { haystack: haystack, re: self, start: 0 }
    }
}
struct MatchesOverlapping<'r, 'h> {
    haystack: &'h str,
    re: &'r regex::Regex,
    start: usize,
}

impl<'r, 'h> Iterator for MatchesOverlapping<'r, 'h> {
    type Item = regex::Match<'h>;

    #[inline]
    fn next(&mut self) -> Option<regex::Match<'h>> {
        match self.re.find_at(self.haystack, self.start){
            None => None,
            Some(new_match) => {
                self.start = new_match.start()+1;
                Some(new_match)
            }
        }
    }
}

advent_of_code::solution!(2);
use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum1: u32 = 0;
    let re_game_id: Regex = Regex::new(r"Game (?P<ID>\d+)").unwrap();
    let re_green: Regex = Regex::new(r"(?P<Count>\d+) green").unwrap();
    let re_blue: Regex = Regex::new(r"(?P<Count>\d+) blue").unwrap();
    let re_red: Regex = Regex::new(r"(?P<Count>\d+) red").unwrap();
    for mut line in input.split("\n").filter(|&x| !x.is_empty()) {
        let mut valid_game = true;
        line = line.trim();
        let (game_id_text,game_list) = match line.split(':').collect::<Vec<&str>>().split_first() {
            Some(s) => {(s.0.to_owned(), s.1.first().unwrap().to_owned())},
            None => panic!("Invalid line"),
        };
        let game_id:u32 = re_game_id.captures(game_id_text).unwrap().name("ID").unwrap().as_str().parse::<u32>().unwrap();
        for game_result in game_list.split(";").filter(|&x| !x.is_empty()){
            let green = match re_green.captures(game_result){
                Some(c) => c.name("Count").unwrap().as_str().parse::<u32>().unwrap(),
                None => 0,
            };
            if green > 13 {
                valid_game = false;
            }

            let blue = match re_blue.captures(game_result){
                Some(c) => c.name("Count").unwrap().as_str().parse::<u32>().unwrap(),
                None => 0,
            };
            if blue > 14 {
                valid_game = false;
            }

            let red = match re_red.captures(game_result){
                Some(c) => c.name("Count").unwrap().as_str().parse::<u32>().unwrap(),
                None => 0,
            };
            if red > 12 {
                valid_game = false;
            }
        }
        if valid_game {
            sum1+= game_id;
        }
    }
    Some(sum1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum2: u32 = 0;
    let re_green: Regex = Regex::new(r"(?P<Count>\d+) green").unwrap();
    let re_blue: Regex = Regex::new(r"(?P<Count>\d+) blue").unwrap();
    let re_red: Regex = Regex::new(r"(?P<Count>\d+) red").unwrap();
    for mut line in input.split("\n").filter(|&x| !x.is_empty()) {
        let mut green_max = 1;
        let mut blue_max = 1;
        let mut red_max = 1;
        line = line.trim();
        let game_list = match line.split(':').collect::<Vec<&str>>().split_first() {
            Some(s) => s.1.first().unwrap().to_owned(),
            None => panic!("Invalid line"),
        };
        for game_result in game_list.split(";"){
            let green = match re_green.captures(game_result){
                Some(c) => c.name("Count").unwrap().as_str().parse::<u32>().unwrap(),
                None => 0,
            };
            if green > green_max {
                green_max = green;
            }

            let blue = match re_blue.captures(game_result){
                Some(c) => c.name("Count").unwrap().as_str().parse::<u32>().unwrap(),
                None => 0,
            };
            if blue > blue_max {
                blue_max = blue;
            }

            let red = match re_red.captures(game_result){
                Some(c) => c.name("Count").unwrap().as_str().parse::<u32>().unwrap(),
                None => 0,
            };
            if red > red_max {
                red_max = red;
            }
        }
        sum2 += red_max*green_max*blue_max;
    }
    Some(sum2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}

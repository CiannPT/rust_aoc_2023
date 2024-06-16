use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut mult:u32 = 1;
    let (times, distances) = match input.split_terminator("\n").collect_vec().split_first() {
        Some(splited) => (splited.0.split(":").last().unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect_vec(),
                                            splited.1[0].split(":").last().unwrap().split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect_vec()),
        None => panic!("Bad parsing"),
    };
    let games = times.iter().zip(distances.iter()).map(|td| (td.0.to_owned(), td.1.to_owned())).collect::<Vec<(u32,u32)>>();
    // println!("{:?}", games);
    for (game_time, game_dist) in games {
        let mut total_distance = 0;
        let mut lowest_win = 0;
        let mut highest_win = game_time;
        while total_distance <= game_dist && lowest_win<game_time{
            lowest_win += 1;
            total_distance = (game_time-lowest_win)*(lowest_win);
        }
        total_distance = 0;
        while total_distance <= game_dist && highest_win>lowest_win{
            highest_win -= 1;
            total_distance = (game_time-highest_win)*(highest_win);
        }
        // println!("{} - {} - {}", lowest_win, highest_win, highest_win-lowest_win+1);
        mult *= highest_win-lowest_win+1;
    }
    Some(mult)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (game_time, game_dist) = match input.split_terminator("\n").collect_vec().split_first() {
        Some(splited) => (splited.0.split(":").last().unwrap().replace(" ", "").parse::<u64>().unwrap(),
                        splited.1[0].split(":").last().unwrap().replace(" ", "").parse::<u64>().unwrap()),
        None => panic!("Bad parsing"),
    };
    // println!("{:?}", games);
    let mut total_distance = 0;
    let mut lowest_win = 0;
    let mut highest_win = game_time;
    while total_distance <= game_dist && lowest_win<game_time{
        lowest_win += 1;
        total_distance = (game_time-lowest_win)*(lowest_win);
    }
    total_distance = 0;
    while total_distance <= game_dist && highest_win>lowest_win{
        highest_win -= 1;
        total_distance = (game_time-highest_win)*(highest_win);
    }
    // println!("{} - {} - {}", lowest_win, highest_win, highest_win-lowest_win+1);
    Some((highest_win-lowest_win+1).try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}

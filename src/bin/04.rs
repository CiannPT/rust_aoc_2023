advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum:u32 =0;
    for line in input.split("\n").filter(|&x| !x.is_empty()) {
        let game_info = line.split(':').collect::<Vec<&str>>()[1];
        let (winning, chosen) = match game_info.split('|').map(|s| s.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>().split_first() {
                Some(v) => {(v.0.to_owned(), v.1.first().unwrap().to_owned())},
                None => panic!("Invalid line"),
            };
        let count:u32 = chosen.into_iter().map(|val| winning.contains(&val) as u32).sum();
        if count >0{
            sum+=u32::pow(2,count-1);
        }
    }
    println!("{}", sum);
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut card_count:Vec<u32> = vec![1];
    let mut last_game_number:usize = 1;
    for line in input.split("\n").filter(|&x| !x.is_empty()) {
        let (game_number, game_info) = match line.split(':').collect::<Vec<&str>>().split_first() {
            Some(s) => {(s.0.split_whitespace().last().unwrap().parse::<usize>().unwrap(), s.1.to_owned().first().unwrap().to_owned())},
            None => panic!("Invalid game"),
        };
        let (winning, chosen) = match game_info.split('|').map(|s| s.split_whitespace().map(|n| n.parse::<u32>().unwrap()).collect::<Vec<u32>>())
            .collect::<Vec<Vec<u32>>>().split_first() {
                Some(v) => {(v.0.to_owned(), v.1.first().unwrap().to_owned())},
                None => panic!("Invalid game info"),
            };
        let points:u32 = chosen.into_iter().map(|val| winning.contains(&val) as u32).sum();
        if card_count.len() < game_number {
            card_count.push(1);
        }
        if points >0{
            for card_number in game_number..(game_number+points as usize) {
                if card_count.len() < card_number+1 {
                    card_count.push(1);
                }
                card_count[card_number]+=card_count[game_number-1];
            }
        }
        last_game_number = game_number;
    }
    while card_count.len() > last_game_number {
        card_count.pop();
    }
    let sum = card_count.into_iter().sum();
    println!("{}", sum);
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}

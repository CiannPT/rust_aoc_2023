advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut hands_list:Vec<Hand> = vec![];
    for line in input.split("\n").filter(|&x| !x.is_empty()) {
        let line_hand = line.split_whitespace().collect::<Vec<&str>>();
        hands_list.push(Hand::new(line_hand[0], line_hand[1], false));
    }
    hands_list.sort();
    for i in 0..hands_list.len() {
        sum+=hands_list[i].bid*(i as u32+1);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    let mut hands_list:Vec<Hand> = vec![];
    for line in input.split("\n").filter(|&x| !x.is_empty()) {
        let line_hand = line.split_whitespace().collect::<Vec<&str>>();
        hands_list.push(Hand::new(line_hand[0], line_hand[1], true));
    }
    hands_list.sort();
    for i in 0..hands_list.len() {
        sum+=hands_list[i].bid*(i as u32+1);
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}


struct Hand {
    cards:[u8;5],
    bid:u32,
    hand_type:u8,
}

impl Hand {
    pub fn new(cards_str: &str, bid: &str, joker:bool) -> Hand {
        let mut new_cards: [u8;5] = [0;5];
        let mut index: usize = 0;
        for card in cards_str.chars(){
            new_cards[index] = match card {
                '1'|'0' => panic!("Invalid card"),
                c if c.is_numeric() => c.to_string().parse::<u8>().unwrap(),
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => if !joker { 11 } else { 1 },
                'T' => 10,
                _ => panic!("Invalid card"),
            };
            index+=1;
        }
        Hand {cards:new_cards, bid:bid.parse::<u32>().unwrap(), hand_type: Hand::get_type(new_cards, joker)}
    }

    fn get_type(cards: [u8;5], joker:bool) -> u8 {
        let mut count: Vec<_> = vec![];
        let joker_points = if joker { cards.iter().filter(|&c| *c==1).count() } else { 0 };
        for card in 2..=14 {
            count.push(cards.iter().filter(|&c| *c==card).count());
        };
        count.sort();
        match count {
            c if c[12]+joker_points == 5 => 6,
            c if c[12]+joker_points == 4 => 5,
            c if c[12]+joker_points == 3 && c[11] == 2 => 4,
            c if c[12]+joker_points == 3 => 3,
            c if c[12]+joker_points == 2 && c[11] == 2 => 2,
            c if c[12]+joker_points == 2 => 1,
            _ => 0
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        }
        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return self.cards[i].cmp(&other.cards[i]);
            }
        }
        return std::cmp::Ordering::Equal;
    }
}

impl Eq for Hand {

}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        if self.hand_type != other.hand_type {
            return false;
        }
        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return false;
            }
        }
        return true;
    }
}

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i32> {
    let mut sum = 0;
    for line in input.split("\n").filter(|&x| !x.is_empty()) {
        let mut val_history: Vec<Vec<i32>> = vec![line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect()];
        while val_history.last().unwrap().into_iter().any(|&n| n != 0) {
            val_history.push(calculate_diferences(val_history.last().unwrap()));
        }
        let mut previous = val_history.pop().unwrap();
        while let Some(mut next) = val_history.pop() {
            next.push(next.last().unwrap() + previous.last().unwrap());
            previous = next;
        }
        // println!("{:?}", previous);
        sum += previous.last().unwrap();
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut sum = 0;
    for line in input.split("\n").filter(|&x| !x.is_empty()) {
        let mut val_history: Vec<Vec<i32>> = vec![line
            .split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect()];
        while val_history.last().unwrap().into_iter().any(|&n| n != 0) {
            val_history.push(calculate_diferences(val_history.last().unwrap()));
        }
        let mut previous = 0;
        while let Some(next) = val_history.pop() {
            previous = next[0]-previous;
        }
        // println!("{:?}", previous);
        sum += previous;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}

fn calculate_diferences<T: std::ops::Sub<Output = T> + Copy>(source: &Vec<T>) -> Vec<T> {
    let mut diferences: Vec<T> = vec![];
    for i in 0..source.len() - 1 {
        diferences.push(source[i + 1] - source[i]);
    }
    return diferences;
}

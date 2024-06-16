use std::cmp;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let char_map = input.split("\n").filter(|&x| !x.is_empty()).map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let total_rows = char_map.len();
    let mut current_part:Part = Part::default();
    let mut sum: u32 = 0;
    let symbol_chars = vec!['.'];
    for current_row in 0..total_rows {
        let row_len = char_map[current_row].len();
        for current_col in 0..row_len{
            if char_map[current_row][current_col].is_numeric() {
                if current_part.number == "".to_string(){
                    current_part.number = char_map[current_row][current_col].to_string();
                    current_part.start_pos = (cmp::max(current_row, 1)-1, cmp::max(current_col, 1)-1);
                    current_part.end_pos = (current_row+1, current_col+1);
                }
                else {
                    current_part.number.push(char_map[current_row][current_col]);
                    current_part.end_pos = (current_row+1, current_col+1);
                }
                // print!("{}", char_map[current_row][current_col]);
            }
            else {
                if current_part.number != "".to_string() {
                    if has_neighbors(&current_part, &char_map, &symbol_chars) {
                        sum += current_part.number.parse::<u32>().unwrap();
                    }
                    current_part.number = "".to_string();
                }
                // print!(" ");
            }
        }
        // println!("");
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let char_map = input.split("\n").filter(|&x| !x.is_empty()).map(|s| s.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let total_rows = char_map.len();
    let mut sum: u32 = 0;
    for current_row in 0..total_rows {
        let row_len = char_map[current_row].len();
        for current_col in 0..row_len{
            if char_map[current_row][current_col] == '*' {
                match gear_ratio((current_row, current_col), &char_map) {
                    Some(x) => sum+=x,
                    None => ()
                }
            }
        }
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}

#[derive(Debug, Default)]
struct Part {
    number: String,
    start_pos: (usize, usize),
    end_pos: (usize, usize)
}

fn has_neighbors(part: &Part, map: &Vec<Vec<char>>, exclude_chars: &Vec<char>) -> bool {
    for row in part.start_pos.0 .. part.end_pos.0+1 {
        if row>= map.len(){
            continue;
        }
        for col in part.start_pos.1 .. part.end_pos.1+1 {
            if col>= map[row].len(){
                continue;
            }
            if !map[row][col].is_numeric() && !exclude_chars.contains(&(map[row][col])) {
                return true;
            }
        }
    }
    false
}

fn gear_ratio(pos: (usize, usize), map: &Vec<Vec<char>>) -> Option<u32> {
    let mut ratios: Vec<String> = Vec::new();
    let mut count:usize = 0;
    let mut number:String;
    let mut end_col:usize = 0;
    for row in cmp::max(pos.0, 1)-1 .. pos.0+2{
        for col in cmp::max(pos.1, 1)-1 .. pos.1+2{
            if map[row][col].is_numeric() && col>end_col {
                (number,end_col) = find_whole_number((row,col), map);
                ratios.push(number);
                count += 1;
                if count >2 {
                    return None;
                }
            }
        }
        end_col = 0;
    }
    if count == 2 {
        return Some(ratios[0].parse::<u32>().unwrap() * ratios[1].parse::<u32>().unwrap());
    }
    None
}

fn find_whole_number(pos: (usize, usize), map: &Vec<Vec<char>>) -> (String, usize) {
    let mut start_col = pos.1;
    let mut end_col = pos.1;
    while start_col>0 && map[pos.0][start_col-1].is_numeric() {
        start_col = cmp::max(start_col,1)-1;
    }
    while end_col+1<map[pos.0].len() && map[pos.0][end_col+1].is_numeric() {
        end_col += 1;
    }
    let mut number = "".to_string();
    for col in start_col..=end_col {
        number.push(map[pos.0][col].clone());
    }
    return (number,end_col);
}

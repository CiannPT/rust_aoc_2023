use std::cmp;
use queues::*;
use my_map::*;

const GROUND:char = '·';
const NS:char = '║';
const NE:char = '╚';
const NW:char = '╝';
const SE:char = '╔';
const SW:char = '╗';
const EW:char = '═';

advent_of_code::solution!(10);


pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::new(input);
    let mut tile_list = queue![map[map.start_pos.unwrap()].clone()];
    let mut max_dist = 0;
    while tile_list.size()>0 {
        let tile = tile_list.remove().unwrap();
        if tile.north && tile.pos.0 > 0{
            let curr_pos = (tile.pos.0-1,tile.pos.1);
            if map[curr_pos].south {
                map[curr_pos].update_dist(tile.distance.unwrap()+1);
                if !map[curr_pos].visited {
                    map[curr_pos].visited = true;
                    let _ = tile_list.add(map[curr_pos].clone());
                }
            }
        }
        if tile.south && tile.pos.0 < map.size.0{
            let curr_pos = (tile.pos.0+1,tile.pos.1);
            if map[curr_pos].north {
                map[curr_pos].update_dist(tile.distance.unwrap()+1);
                if !map[curr_pos].visited {
                    map[curr_pos].visited = true;
                    let _ = tile_list.add(map[curr_pos].clone());
                }
            }
        }
        if tile.west && tile.pos.1 > 0{
            let curr_pos = (tile.pos.0,tile.pos.1-1);
            if map[curr_pos].east {
                map[curr_pos].update_dist(tile.distance.unwrap()+1);
                if !map[curr_pos].visited {
                    map[curr_pos].visited = true;
                    let _ = tile_list.add(map[curr_pos].clone());
                }
            }
        }
        if tile.east && tile.pos.1 < map.size.1{
            let curr_pos = (tile.pos.0,tile.pos.1+1);
            if map[curr_pos].west {
                map[curr_pos].update_dist(tile.distance.unwrap()+1);
                if !map[curr_pos].visited {
                    map[curr_pos].visited = true;
                    let _ = tile_list.add(map[curr_pos].clone());
                }
            }
        }
        max_dist = cmp::max(max_dist, tile.distance.unwrap())
    }

    Some(max_dist)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::new(input);
    let start_pos = map.start_pos.unwrap();
    let mut tile_list = queue![map[start_pos].clone()];

    if !map[(start_pos.0-1,start_pos.1)].south { map[start_pos].north = false}
    while tile_list.size()>0 {
        let tile = tile_list.remove().unwrap();
        if tile.north && tile.pos.0 > 0{
            let curr_pos = (tile.pos.0-1,tile.pos.1);
            if map[curr_pos].south && !map[curr_pos].visited {
                map[curr_pos].visited = true;
                let _ = tile_list.add(map[curr_pos].clone());
            }
        }
        if tile.south && tile.pos.0 < map.size.0{
            let curr_pos = (tile.pos.0+1,tile.pos.1);
            if map[curr_pos].north && !map[curr_pos].visited {
                map[curr_pos].visited = true;
                let _ = tile_list.add(map[curr_pos].clone());
            }
        }
        if tile.west && tile.pos.1 > 0{
            let curr_pos = (tile.pos.0,tile.pos.1-1);
            if map[curr_pos].east && !map[curr_pos].visited {
                map[curr_pos].visited = true;
                let _ = tile_list.add(map[curr_pos].clone());
            }
        }
        if tile.east && tile.pos.1 < map.size.1{
            let curr_pos = (tile.pos.0,tile.pos.1+1);
            if map[curr_pos].west && !map[curr_pos].visited {
                map[curr_pos].visited = true;
                let _ = tile_list.add(map[curr_pos].clone());
            }
        }
    }

    for r in 0..map.size.0 {
        for c in 0..map.size.1{
            if !map[(r,c)].visited {
                map[(r,c)].symbol = GROUND;
                map[(r,c)].north = false;
            }
        }
    }
    let mut inside_points = 0;
    for r in 0..map.size.0 {
        for c in 0..map.size.1{
            if map[(r,c)].symbol==GROUND {
                let walls = ray_cast(map.get_range((r,0), (r,c)));
                if walls %2 ==1 {
                    map[(r,c)].symbol = '■';
                    inside_points +=1;
                }
            }
        }
    }
    //println!("{}",map);
    Some(inside_points)
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
        assert_eq!(result, Some(8));
    }
}


mod my_map{
    use core::fmt;
    use std::{cmp, ops::{Index, IndexMut}};

    use crate::{EW, GROUND, NE, NS, NW, SE, SW};

    #[derive(Copy, Clone)]
    pub struct Tile {
        pub symbol:char,
        pub north:bool,
        pub south:bool,
        pub east:bool,
        pub west:bool,
        pub distance:Option<u32>,
        pub pos:(usize,usize),
        pub visited:bool,
    }
    impl Tile {
        pub fn new(symbol: &char, pos:(usize,usize)) -> Tile{
            match symbol {
                '|' => Tile{symbol:NS,north:true,south:true,east:false,west:false,distance:None,pos:pos,visited:false},
                '-' => Tile{symbol:EW,north:false,south:false,east:true,west:true,distance:None,pos:pos,visited:false},
                'L' => Tile{symbol:NE,north:true,south:false,east:true,west:false,distance:None,pos:pos,visited:false},
                'J' => Tile{symbol:NW,north:true,south:false,east:false,west:true,distance:None,pos:pos,visited:false},
                '7' => Tile{symbol:SW,north:false,south:true,east:false,west:true,distance:None,pos:pos,visited:false},
                'F' => Tile{symbol:SE,north:false,south:true,east:true,west:false,distance:None,pos:pos,visited:false},
                '.' => Tile{symbol:GROUND,north:false,south:false,east:false,west:false,distance:None,pos:pos,visited:false},
                'S' => Tile{symbol:symbol.to_owned(),north:true,south:true,east:true,west:true,distance:Some(0),pos:pos,visited:false},
                _ => panic!("Unkown symbol {symbol}")
            }
        }

        pub fn update_dist(&mut self, new_dist: u32){
            match self.distance {
                Some(dist) => self.distance = Some(cmp::min(dist,new_dist)),
                None => self.distance = Some(new_dist)
            };
        }
    }

    #[derive(Clone)]
    pub struct Map {
        tile_array: Vec<Tile>,
        pub start_pos:Option<(usize,usize)>,
        pub size:(usize,usize),
    }

    impl Map {
        pub fn new(input: &str) -> Map {
            let mut vec_map = Vec::<Tile>::new();
            let mut rows = 0usize;
            let mut pos:Option<(usize,usize)> = None;
            for line in input.split("\n").filter(|&x| !x.is_empty())  {
                let mut col = 0usize;
                for c in line.chars() {
                    vec_map.push(Tile::new(&c, (rows,col)));
                    if c=='S' {
                        pos = Some((rows,col));
                    }
                    col+=1;
                }
                rows+=1;
            }
            let cols = vec_map.len()/rows;

            Map{tile_array:vec_map,start_pos:pos, size:(rows,cols)}
        }

        pub fn get_range(&self,start: (usize,usize),end:(usize,usize)) -> Vec<&Tile>{
            let mut out_range = Vec::<&Tile>::new();
            for r in start.0..=(end.0) {
                for c in start.1..=(end.1) {
                    out_range.push(&self[(r,c)]);
                }
            }
            return out_range;
        }
    }

    impl Index<(usize, usize)> for Map {
        type Output = Tile;

        #[inline(always)]
        fn index(&self, index: (usize, usize)) -> &Self::Output {
            self.tile_array.index(index.0*self.size.1+index.1)
        }
    }

    impl IndexMut<(usize, usize)> for Map {

        #[inline(always)]
        fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
            self.tile_array.index_mut(index.0*self.size.1+index.1)
        }
    }

    impl fmt::Display for Map {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for r in 0..self.size.0 {
                for c in 0..self.size.1 {
                    match write!(f,"{}",self[(r,c)].symbol) {
                        Err(err) => return Err(err),
                        _ => (),
                    };
                }
                match write!(f,"\n") {
                    Err(err) => return Err(err),
                    _ => (),
                };
            }
            return Ok(());
        }
    }

    pub fn ray_cast(slice: Vec<&Tile>) -> u32 {
        let mut walls = 0;
        for tile in slice {
            if tile.north{
                walls += 1;
            }
        }
        return walls;
    }

}

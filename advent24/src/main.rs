#![feature(split_inclusive)]
use std::collections::HashSet;

// reuse code from day 17
mod ndlife;
use ndlife::{run_nd_life, HasNeighbors, LifeParams};

const PARAMS: LifeParams = LifeParams {
    min_survival: 1,
    max_survival: 2,
    birth: 2,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct HexCoordinate {
    q: i32,
    r: i32,
}

impl HexCoordinate {
    fn origin() -> Self {
        Self { q: 0, r: 0 }
    }

    fn move_e(&self) -> Self {
        Self {
            q: self.q + 1,
            r: self.r,
        }
    }

    fn move_w(&self) -> Self {
        Self {
            q: self.q - 1,
            r: self.r,
        }
    }

    fn move_ne(&self) -> Self {
        Self {
            q: self.q + 1,
            r: self.r - 1,
        }
    }

    fn move_sw(&self) -> Self {
        Self {
            q: self.q - 1,
            r: self.r + 1,
        }
    }

    fn move_nw(&self) -> Self {
        Self {
            q: self.q,
            r: self.r - 1,
        }
    }

    fn move_se(&self) -> Self {
        Self {
            q: self.q,
            r: self.r + 1,
        }
    }
}

impl HasNeighbors for HexCoordinate {
    fn neighbors(self) -> Vec<Self> {
        vec![
            self.move_e(),
            self.move_se(),
            self.move_sw(),
            self.move_w(),
            self.move_nw(),
            self.move_ne(),
        ]
    }
}

fn string_to_hex(input: &str) -> HexCoordinate {
    let mut coord = HexCoordinate::origin();
    let directions = input.split_inclusive(|ch| ch == 'e' || ch == 'w');
    for piece in directions {
        match piece {
            "e" => coord = coord.move_e(),
            "w" => coord = coord.move_w(),
            "ne" => coord = coord.move_ne(),
            "nw" => coord = coord.move_nw(),
            "se" => coord = coord.move_se(),
            "sw" => coord = coord.move_sw(),
            _ => panic!("Unfamiliar direction: {}", piece),
        }
    }
    coord
}

fn flip_hex_tiles_from_input(lines: &Vec<&str>) -> HashSet<HexCoordinate> {
    let mut flipped: HashSet<HexCoordinate> = HashSet::new();
    for &line in lines {
        let hex = string_to_hex(line);
        if flipped.contains(&hex) {
            flipped.remove(&hex);
        } else {
            flipped.insert(hex);
        }
    }
    flipped
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let state = flip_hex_tiles_from_input(&lines);
    println!("Tiles flipped in start configuration: {}", state.len());

    let state = run_nd_life(state, 100, PARAMS);
    println!("{} cells alive at end", state.len());
}

#[cfg(test)]
mod test {
    use super::*;
    use ndlife::step_nd_life;

    #[test]
    fn test_life_steps() {
        let start_directions: Vec<&str> = vec![
            "sesenwnenenewseeswwswswwnenewsewsw",
            "neeenesenwnwwswnenewnwwsewnenwseswesw",
            "seswneswswsenwwnwse",
            "nwnwneseeswswnenewneswwnewseswneseene",
            "swweswneswnenwsewnwneneseenw",
            "eesenwseswswnenwswnwnwsewwnwsene",
            "sewnenenenesenwsewnenwwwse",
            "wenwwweseeeweswwwnwwe",
            "wsweesenenewnwwnwsenewsenwwsesesenwne",
            "neeswseenwwswnwswswnw",
            "nenwswwsewswnenenewsenwsenwnesesenew",
            "enewnwewneswsewnwswenweswnenwsenwsw",
            "sweneswneswneneenwnewenewwneswswnese",
            "swwesenesewenwneswnwwneseswwne",
            "enesenwswwswneneswsenwnewswseenwsese",
            "wnwnesenesenenwwnenwsewesewsesesew",
            "nenewswnwewswnenesenwnesewesw",
            "eneswnwswnwsenenwnwnwwseeswneewsenese",
            "neswnwewnwnwseenwseesewsenwsweewe",
            "wseweeenwnesenwwwswnew",
        ];
        let mut state = flip_hex_tiles_from_input(&start_directions);
        assert_eq!(state.len(), 10);
        state = step_nd_life(state, PARAMS);
        assert_eq!(state.len(), 15);
        state = step_nd_life(state, PARAMS);
        assert_eq!(state.len(), 12);
        state = step_nd_life(state, PARAMS);
        assert_eq!(state.len(), 25);
    }
}

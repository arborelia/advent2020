use defaultmap::DefaultHashMap;
use scan_fmt::scan_fmt;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;

#[derive(Debug, Clone, Eq)]
pub struct Tile {
    id: u32,

    // each edge of the tile, as a bit pattern
    top: u32,
    bottom: u32,
    left: u32,
    right: u32,
    image: Vec<Vec<char>>,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Tile {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

/// Reverse a 10-bit number representing an edge.
fn reverse(num: u32) -> u32 {
    let mut val: u32 = 0;
    for bit in 0..10 {
        if num & (1 << bit) != 0 {
            val += 1 << (9 - bit);
        }
    }
    val
}

#[derive(Debug, Clone)]
pub struct OrientedTile {
    tile: Tile,
    flipped_h: bool,
    flipped_v: bool,
    turned_right: bool,
}

impl OrientedTile {
    fn oriented_edges(&self) -> (u32, u32, u32, u32) {
        let mut top: u32 = self.tile.top;
        let mut right: u32 = self.tile.right;
        let mut bottom: u32 = self.tile.bottom;
        let mut left: u32 = self.tile.left;

        if self.turned_right {
            let tmp = left;
            left = bottom;
            bottom = right;
            right = top;
            top = tmp;
        }
        if self.flipped_v {
            let tmp = top;
            top = reverse(bottom);
            bottom = reverse(tmp);
            left = reverse(left);
            right = reverse(right);
        }
        if self.flipped_h {
            let tmp = left;
            left = reverse(right);
            right = reverse(tmp);
            top = reverse(top);
            bottom = reverse(bottom);
        }
        (top, right, bottom, left)
    }

    pub fn top_edge(&self) -> u32 {
        self.oriented_edges().0
    }
    pub fn right_edge(&self) -> u32 {
        self.oriented_edges().1
    }
    pub fn bottom_edge(&self) -> u32 {
        self.oriented_edges().2
    }
    pub fn left_edge(&self) -> u32 {
        self.oriented_edges().3
    }
}

fn read_tiles_from_file(path: &str) -> Vec<Tile> {
    let input = std::fs::read_to_string(path).unwrap();
    let mut tiles: Vec<Tile> = Vec::new();
    let mut tile_num: u32 = 0;
    let mut rows: Vec<Vec<char>> = Vec::new();
    for line in input.split("\n") {
        if line.len() == 0 {
            if rows.len() > 0 {
                // interpret the existing rows as a 10x10 tile, and note the edges
                let mut top: u32 = 0;
                let mut bottom: u32 = 0;
                let mut left: u32 = 0;
                let mut right: u32 = 0;
                for pos in 0..10 {
                    if rows[0][pos] == '#' {
                        top += 1 << pos;
                    }
                    if rows[9][9 - pos] == '#' {
                        bottom += 1 << pos;
                    }
                    if rows[9 - pos][0] == '#' {
                        left += 1 << pos;
                    }
                    if rows[pos][9] == '#' {
                        right += 1 << pos;
                    }
                }
                let tile = Tile {
                    id: tile_num,
                    top,
                    bottom,
                    left,
                    right,
                    image: rows.clone(),
                };
                println!("{:?}", tile);
                tiles.push(tile);
                rows.clear();
            }
        } else {
            match scan_fmt!(line, "Tile {d}:", u32) {
                Ok(num) => tile_num = num,
                _ => {
                    rows.push(line.chars().collect());
                }
            }
        }
    }
    println!("read {} tiles", tiles.len());
    tiles
}

fn get_edge_counts(tiles: &[Tile]) -> DefaultHashMap<u32, u32> {
    let mut edge_counts: DefaultHashMap<u32, u32> = DefaultHashMap::new(0);
    for tile in tiles {
        edge_counts[tile.top] += 1;
        edge_counts[tile.bottom] += 1;
        edge_counts[tile.left] += 1;
        edge_counts[tile.right] += 1;
        edge_counts[reverse(tile.top)] += 1;
        edge_counts[reverse(tile.bottom)] += 1;
        edge_counts[reverse(tile.left)] += 1;
        edge_counts[reverse(tile.right)] += 1;
    }
    edge_counts
}

const N: usize = 12;

fn find_tile(
    grid: &Vec<Vec<Option<OrientedTile>>>,
    tileset: &HashSet<Tile>,
    edge_counts: &DefaultHashMap<u32, u32>,
    row: usize,
    col: usize,
) -> Option<OrientedTile> {
    // find a tile that fits this position
    let mut found_tile: Option<OrientedTile> = None;
    for tile in tileset.iter() {
        for &turned_right in [false, true].iter() {
            for &flipped_h in [false, true].iter() {
                for &flipped_v in [false, true].iter() {
                    let oriented: OrientedTile = OrientedTile {
                        tile: tile.clone(),
                        flipped_h,
                        flipped_v,
                        turned_right,
                    };
                    let mut is_ok = true;
                    if row == 0 {
                        if edge_counts[oriented.top_edge()] > 1 {
                            // not a top edge tile
                            is_ok = false;
                        }
                    } else {
                        let tile_above = &grid[row - 1][col];
                        if let Some(otile) = tile_above {
                            if reverse(otile.bottom_edge()) != oriented.top_edge() {
                                is_ok = false;
                            }
                        }
                    }
                    if col == 0 {
                        if edge_counts[oriented.left_edge()] > 1 {
                            is_ok = false;
                        }
                    } else {
                        let tile_left = &grid[row][col - 1];
                        if let Some(otile) = tile_left {
                            if reverse(otile.right_edge()) != oriented.left_edge() {
                                is_ok = false;
                            }
                        }
                    }
                    if is_ok {
                        found_tile = Some(oriented.clone());
                    }
                }
            }
        }
        if found_tile.is_some() {
            break;
        }
    }
    found_tile
}

fn show_grid(grid: &Vec<Vec<Option<OrientedTile>>>) {
    for row in 0..N {
        for col in 0..N {
            match &grid[row][col] {
                Some(otile) => {
                    print!("{} ", otile.tile.id);
                }
                None => {
                    print!("---- ");
                }
            }
        }
        println!("");
    }
    println!("");
}

fn main() {
    let tiles = read_tiles_from_file("input.txt");
    let mut tileset: HashSet<Tile> = HashSet::from_iter(tiles.iter().cloned());

    let edge_counts = get_edge_counts(&tiles);
    let mut grid: Vec<Vec<Option<OrientedTile>>> = Vec::new();
    for _row in 0..N {
        grid.push(vec![None; N]);
    }

    for row in 0..N {
        for col in 0..N {
            let otile: Option<OrientedTile> = find_tile(&grid, &tileset, &edge_counts, row, col);
            if let Some(otile) = otile.clone() {
                tileset.remove(&otile.tile);
            }
            grid[row][col] = otile;
            show_grid(&grid);
        }
    }
    let corners: Vec<OrientedTile> = vec![
        grid[0][0].as_ref().unwrap().clone(),
        grid[0][N - 1].as_ref().unwrap().clone(),
        grid[N - 1][0].as_ref().unwrap().clone(),
        grid[N - 1][N - 1].as_ref().unwrap().clone(),
    ];
    let product: u64 = corners.iter().map(|otile| otile.tile.id as u64).product();
    println!("corner product: {}", product);
}

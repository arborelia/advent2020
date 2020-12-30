use array2d::Array2D;
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

    // the . and # chars that make up the tile
    image: Array2D<char>,
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

fn array_rotate_right(arr: Array2D<char>) -> Array2D<char> {
    // 0 1 2      6 3 0
    // 3 4 5  ->  7 4 1
    // 6 7 8      8 5 2
    let rev_rows: Vec<Vec<char>> = arr.as_rows().iter().rev().cloned().collect();
    Array2D::from_columns(&rev_rows)
}

fn array_flip_v(arr: Array2D<char>) -> Array2D<char> {
    let rev_rows: Vec<Vec<char>> = arr.as_rows().iter().rev().cloned().collect();
    Array2D::from_rows(&rev_rows)
}

fn array_flip_h(arr: Array2D<char>) -> Array2D<char> {
    let rev_cols: Vec<Vec<char>> = arr.as_columns().iter().rev().cloned().collect();
    Array2D::from_columns(&rev_cols)
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

    /// Get the piece of the image represented by this tile in this orientation,
    /// with the border removed.
    pub fn image(&self) -> Array2D<char> {
        let mut arr: Array2D<char> = self.tile.image.clone();

        if self.turned_right {
            arr = array_rotate_right(arr);
        }
        if self.flipped_h {
            arr = array_flip_h(arr);
        }
        if self.flipped_v {
            arr = array_flip_v(arr);
        }

        let mut trim_arr: Array2D<char> =
            Array2D::filled_with('.', arr.num_rows() - 2, arr.num_columns() - 2);
        for row in 1..(arr.num_rows() - 1) {
            for col in 1..(arr.num_columns() - 1) {
                trim_arr[(row - 1, col - 1)] = arr[(row, col)];
            }
        }
        trim_arr
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
                    image: Array2D::from_rows(&rows),
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

fn assemble_tiles(grid: &Vec<Vec<Option<OrientedTile>>>) -> Array2D<char> {
    let mut image: Array2D<char> = Array2D::filled_with(' ', N * 8, N * 8);
    for row in 0..N {
        for col in 0..N {
            let otile = grid[row][col]
                .clone()
                .expect("There's an unknown tile here");
            let tile_image: Array2D<char> = otile.image();
            for irow in 0..8 {
                for icol in 0..8 {
                    image[(row * 8 + irow, col * 8 + icol)] = tile_image[(irow, icol)];
                }
            }
        }
    }
    image
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

    let image = assemble_tiles(&grid);
    let n_dark: usize = image.as_row_major().iter().filter(|&&c| c == '#').count();
    let sea_monster_vec: Vec<Vec<char>> = vec![
        "                  O ".chars().collect(),
        "O    OO    OO    OOO".chars().collect(),
        " O  O  O  O  O  O   ".chars().collect(),
    ];
    let monster: Array2D<char> = Array2D::from_rows(&sea_monster_vec);

    for &rotate in [true, false].iter() {
        for &flip_v in [true, false].iter() {
            for &flip_h in [true, false].iter() {
                let mut image_t = image.clone();
                if rotate {
                    image_t = array_rotate_right(image_t);
                }
                if flip_v {
                    image_t = array_flip_v(image_t);
                }
                if flip_h {
                    image_t = array_flip_h(image_t);
                }

                let mut monsters_found: u32 = 0;
                for row_offset in 0..=(image.num_rows() - monster.num_rows()) {
                    for col_offset in 0..=(image.num_columns() - monster.num_columns()) {
                        let mut possible_monster: bool = true;
                        for row in 0..monster.num_rows() {
                            if !possible_monster {
                                break;
                            }
                            for col in 0..monster.num_columns() {
                                if monster[(row, col)] == 'O' {
                                    if image_t[(row + row_offset, col + col_offset)] != '#' {
                                        possible_monster = false;
                                        break;
                                    }
                                }
                            }
                        }
                        if possible_monster {
                            monsters_found += 1;
                        }
                    }
                }
                let roughness = n_dark as u32 - monsters_found * 15;
                println!("{} monsters found: roughness {}", monsters_found, roughness);
            }
        }
    }
}

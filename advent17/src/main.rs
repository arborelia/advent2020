use defaultmap::DefaultHashMap;
use std::collections::HashSet;
use std::hash::Hash;

// Define the HasNeighbors trait, so we can generalize over 3D and 4D Life-like
// automatons. And beyond! Except we're not going beyond.

trait HasNeighbors: Sized + Eq + Hash + Copy {
    fn neighbors(self) -> Vec<Self>;
}

impl HasNeighbors for (i32, i32, i32) {
    fn neighbors(self) -> Vec<(i32, i32, i32)> {
        let mut neighbor_list: Vec<(i32, i32, i32)> = Vec::new();
        let (x, y, z) = self;
        for nx in (x - 1)..=(x + 1) {
            for ny in (y - 1)..=(y + 1) {
                for nz in (z - 1)..=(z + 1) {
                    if (nx, ny, nz) != (x, y, z) {
                        neighbor_list.push((nx, ny, nz))
                    }
                }
            }
        }
        neighbor_list
    }
}

impl HasNeighbors for (i32, i32, i32, i32) {
    fn neighbors(self) -> Vec<(i32, i32, i32, i32)> {
        let mut neighbor_list: Vec<(i32, i32, i32, i32)> = Vec::new();
        let (x, y, z, w) = self;
        for nx in (x - 1)..=(x + 1) {
            for ny in (y - 1)..=(y + 1) {
                for nz in (z - 1)..=(z + 1) {
                    for nw in (w - 1)..=(w + 1) {
                        if (nx, ny, nz, nw) != (x, y, z, w) {
                            neighbor_list.push((nx, ny, nz, nw))
                        }
                    }
                }
            }
        }
        neighbor_list
    }
}

fn step_nd_life<T: HasNeighbors>(grid: HashSet<T>) -> HashSet<T> {
    let mut adjacency: DefaultHashMap<T, u32> = DefaultHashMap::new(0);
    let mut newgrid: HashSet<T> = HashSet::new();
    for cell in grid.iter() {
        for neighbor in cell.neighbors() {
            // increment the adjacency count for each neighbor
            adjacency[neighbor] += 1;
        }
    }
    for &cell in grid.iter() {
        let n_neighbors = adjacency[cell];
        if n_neighbors == 2 || n_neighbors == 3 {
            newgrid.insert(cell);
        }
    }
    for (&cell, &n_neighbors) in adjacency.iter() {
        if !grid.contains(&cell) && n_neighbors == 3 {
            newgrid.insert(cell);
        }
    }
    newgrid
}

fn run_nd_life<T: HasNeighbors>(grid: HashSet<T>, nsteps: usize) -> HashSet<T> {
    let mut grid = grid;
    for _step in 0..nsteps {
        println!("Step {}: {} cells", _step, grid.len());
        grid = step_nd_life(grid);
    }
    grid
}

fn main() {
    let mut init_state_3d: HashSet<(i32, i32, i32)> = HashSet::new();
    let mut init_state_4d: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let lines = input.split("\n");
    for (row, line) in lines.enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                init_state_3d.insert((0, row as i32, col as i32));
                init_state_4d.insert((0, 0, row as i32, col as i32));
            }
        }
    }
    let final_state = run_nd_life(init_state_3d, 6);
    println!("cells in 3d: {}", final_state.len());
    let final_state = run_nd_life(init_state_4d, 6);
    println!("cells in 4d: {}", final_state.len());
}

#[test]
fn test_glider_3d() {
    let mut init_state: HashSet<(i32, i32, i32)> = HashSet::new();
    init_state.insert((0, 0, 1));
    init_state.insert((0, 1, 2));
    init_state.insert((0, 2, 0));
    init_state.insert((0, 2, 1));
    init_state.insert((0, 2, 2));
    let final_state = run_nd_life(init_state, 6);
    assert_eq!(final_state.len(), 112);
}

#[test]
fn test_glider_4d() {
    let mut init_state: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    init_state.insert((0, 0, 0, 1));
    init_state.insert((0, 0, 1, 2));
    init_state.insert((0, 0, 2, 0));
    init_state.insert((0, 0, 2, 1));
    init_state.insert((0, 0, 2, 2));
    let final_state = run_nd_life(init_state, 6);
    assert_eq!(final_state.len(), 848);
}

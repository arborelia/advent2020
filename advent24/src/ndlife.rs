use defaultmap::DefaultHashMap;
use std::collections::HashSet;
use std::hash::Hash;

pub trait HasNeighbors: Sized + Eq + Hash + Copy {
    fn neighbors(self) -> Vec<Self>;
}

#[derive(Clone, Copy, Debug)]
pub struct LifeParams {
    pub min_survival: u32,
    pub max_survival: u32,
    pub birth: u32,
}

// Now that we parameterize the rules of Life, this is the code that would have
// been used for the standard Life rules used on day 17:
//
// const ConwayLife: LifeParams = LifeParams {
//     min_survival: 2,
//     max_survival: 3,
//     birth: 3,
// };

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

pub fn step_nd_life<T: HasNeighbors>(grid: HashSet<T>, params: LifeParams) -> HashSet<T> {
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
        if n_neighbors >= params.min_survival && n_neighbors <= params.max_survival {
            newgrid.insert(cell);
        }
    }
    for (&cell, &n_neighbors) in adjacency.iter() {
        if !grid.contains(&cell) && n_neighbors == params.birth {
            newgrid.insert(cell);
        }
    }
    newgrid
}

pub fn run_nd_life<T: HasNeighbors>(
    grid: HashSet<T>,
    nsteps: usize,
    params: LifeParams,
) -> HashSet<T> {
    let mut grid = grid;
    for _step in 0..nsteps {
        println!("Step {}: {} cells", _step, grid.len());
        grid = step_nd_life(grid, params);
    }
    grid
}

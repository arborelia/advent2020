use std::cell::RefCell;
use std::rc::Rc;

/// A circular linked list containing a permutation of the numbers from 1 ..= n.
/// It keeps an index so that you can quickly get a pointer to any element in the list
/// by its value.
pub struct SpinnyList {
    head: Rc<SpinnyListCell>,
    index: Vec<Option<Rc<SpinnyListCell>>>,
}

impl SpinnyList {
    /// Construct a SpinnyList from a slice containing a permutation of the numbers
    /// from 1 to n.
    pub fn from_slice(slice: &[u32]) -> SpinnyList {
        let size = slice.len();

        // Create the index of pointers, which are initially None, and which will be
        // replaced with references to the cell containing each number.
        // (This task uses 1-based numbers from 1 to n, but of course their actual
        // 0-based positions in the index are from 0 to n-1.)
        let mut index: Vec<Option<Rc<SpinnyListCell>>> = vec![None; size];

        // This is a place to store a pointer to the last cell, once we've constructed
        // the last cell, so that we can quickly find it again and glue it to the first
        // cell.
        let mut end_of_list: Option<Rc<SpinnyListCell>> = None;

        // Build up the list from the end to the start, with each new cell pointing
        // to the chain we've made from the cells so far.
        let mut current: Option<Rc<SpinnyListCell>> = None;
        for &val in slice.iter().rev() {
            let cell = SpinnyListCell {
                val,
                tail: RefCell::new(current),
            };
            current = Some(Rc::new(cell));

            // `current` is a pointer to the cell we just made.
            // Put another pointer to the cell in the index.
            index[(val - 1) as usize] = current.clone();

            if end_of_list.is_none() {
                end_of_list = current.clone();
            }
        }
        let head = current.clone().unwrap();

        // Attach the end of the list to the head, making it circular.
        let end = end_of_list.unwrap();
        *end.tail.borrow_mut() = current;
        SpinnyList { head, index }
    }

    /// Implements an important operation for this Advent of Code problem.
    /// Look at the next `chunk_size` items after the current head. Find a
    /// destination to move them to, by its value. Remove those items from their
    /// current position and splice them in after the destination value.
    fn move_next_chunk(&mut self, chunk_size: usize, dest_val: u32) {
        let head_cell = self.head.clone();
        let startpoint = head_cell.next();
        let mut penultimate = startpoint.clone();
        let mut values_being_moved: Vec<u32> = vec![startpoint.val];

        // Keep track of which values are being moved. If the destination value
        // is one of these values, we're supposed to decrement the destination
        // (wrapping around) until it isn't.
        for _i in 0..(chunk_size - 1) {
            penultimate = penultimate.next();
            values_being_moved.push(penultimate.val);
        }
        let mut dest_val = dest_val;
        while values_being_moved.contains(&dest_val) {
            dest_val -= 1;
            if dest_val == 0 {
                dest_val = self.len() as u32;
            }
        }

        // Find the destination, as pointers to the destination and the next element.
        let destination = self.find_cell(dest_val).unwrap();
        let destination_next = destination.next();

        // Mutate the pointers to move the segment to the destination.
        let endpoint = penultimate.next();
        *head_cell.tail.borrow_mut() = Some(endpoint.clone());
        *destination.tail.borrow_mut() = Some(startpoint.clone());
        *penultimate.tail.borrow_mut() = Some(destination_next.clone());
    }

    /// Get the first value pointed to by the head of the list.
    pub fn first(&self) -> u32 {
        self.head.as_ref().value()
    }

    /// Use the index to find a cell by value.
    pub fn find_cell(&self, val: u32) -> Option<Rc<SpinnyListCell>> {
        self.index[(val - 1) as usize].clone()
    }

    /// Move the head of the list to the location of a particular value.
    pub fn jump_to(&mut self, val: u32) {
        self.head = self.find_cell(val).expect("missing cell");
    }

    /// Move the head of the list one step forward.
    pub fn step_once(&mut self) {
        self.head = self.head.as_ref().next()
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// "Unwind" the SpinnyList into a vector so that we can examine its contents
    /// in order.
    pub fn to_vec(&mut self) -> Vec<u32> {
        let mut vals: Vec<u32> = Vec::new();
        let mut cell = self.head.clone();
        while vals.len() < self.len() {
            vals.push(cell.value());
            cell = cell.next();
        }
        vals
    }
}

/// The cons-cell structure that forms the interior of a SpinnyList.
pub struct SpinnyListCell {
    val: u32,
    tail: RefCell<Option<Rc<SpinnyListCell>>>,
}

impl SpinnyListCell {
    fn value(&self) -> u32 {
        self.val
    }

    fn next(&self) -> Rc<Self> {
        match *self.tail.borrow() {
            Some(ref tail) => tail.clone(),
            None => panic!("reached the end of the SpinnyList somehow"),
        }
    }
}

/// Perform one move of the cup game.
fn perform_cup_move(cups: &mut SpinnyList) {
    let first_cup: u32 = cups.first();
    let destination: u32 = if first_cup == 1 {
        cups.len() as u32
    } else {
        first_cup - 1
    };
    cups.move_next_chunk(3, destination);
    cups.step_once();
}

/// Perform n moves of the cup game, then return the order of all the cups
/// after 1.
fn order_after_n_moves(cups: &mut SpinnyList, n: u32) -> Vec<u32> {
    for iter in 0..n {
        // see some progress in long runs
        if iter > 0 && iter % 100000 == 0 {
            dbg!(iter);
        }
        perform_cup_move(cups);
    }
    cups.jump_to(1);
    let mut order = cups.to_vec();

    // Remove the 1, which isn't considered part of the order
    order.remove(0);
    order
}

fn main() {
    // Part 1: run the cup game for 100 steps on my particular input.
    let mut cups: SpinnyList = SpinnyList::from_slice(&[5, 8, 9, 1, 7, 4, 2, 6, 3]);
    let order = order_after_n_moves(&mut cups, 100);
    println!("Cup order for part 1: {:?}", order);

    // Part 2: extend the input to a million cups, and run the cup game for
    // 10 million steps.
    let mut cup_vec: Vec<u32> = vec![5, 8, 9, 1, 7, 4, 2, 6, 3];
    for cup in 10..=1_000_000 {
        cup_vec.push(cup);
    }
    let mut cups: SpinnyList = SpinnyList::from_slice(&cup_vec);
    let order = order_after_n_moves(&mut cups, 10_000_000);
    println!("Product for part 2: {}", order[0] * order[1]);
}

#[test]
fn test_construct_spinny_list() {
    let mut spinny = SpinnyList::from_slice(&[1, 3, 2]);
    assert_eq!(spinny.to_vec(), vec![1, 3, 2]);
    assert_eq!(spinny.to_vec(), vec![1, 3, 2]);
}

#[test]
fn test_find_values() {
    let mut spinny = SpinnyList::from_slice(&[5, 1, 3, 2, 4]);
    assert_eq!(spinny.find_cell(1).unwrap().val, 1);
    assert_eq!(spinny.find_cell(2).unwrap().val, 2);
    assert_eq!(spinny.find_cell(3).unwrap().val, 3);
    assert_eq!(spinny.find_cell(4).unwrap().val, 4);

    spinny.jump_to(3);
    assert_eq!(spinny.to_vec(), vec![3, 2, 4, 5, 1]);
    assert_eq!(spinny.find_cell(4).unwrap().val, 4);
}

#[test]
fn test_move_chunk() {
    let mut spinny = SpinnyList::from_slice(&[5, 1, 3, 2, 4, 7, 6]);
    spinny.move_next_chunk(3, 7);
    assert_eq!(spinny.to_vec(), vec![5, 4, 7, 1, 3, 2, 6]);
}

#[test]
fn test_individual_moves() {
    let mut cups: SpinnyList = SpinnyList::from_slice(&[3, 8, 9, 1, 2, 5, 4, 6, 7]);
    perform_cup_move(&mut cups);
    assert_eq!(cups.to_vec(), vec![2, 8, 9, 1, 5, 4, 6, 7, 3]);
    perform_cup_move(&mut cups);
    assert_eq!(cups.to_vec(), vec![5, 4, 6, 7, 8, 9, 1, 3, 2]);
}

#[test]
fn test_100_moves() {
    let mut cups: SpinnyList = SpinnyList::from_slice(&[3, 8, 9, 1, 2, 5, 4, 6, 7]);
    let answer = order_after_n_moves(&mut cups, 100);
    assert_eq!(answer, vec![6, 7, 3, 8, 4, 5, 2, 9]);
}

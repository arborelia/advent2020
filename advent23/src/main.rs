use std::cell::RefCell;
use std::rc::Rc;

/// A circular linked list of consecutive integers, that remembers where each of the
/// numbers in it is as it's rearranged.
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

pub struct SpinnyList {
    head: Option<Rc<SpinnyListCell>>,
    index: Vec<Option<Rc<SpinnyListCell>>>,
}

impl SpinnyList {
    pub fn from_slice(slice: &[u32]) -> SpinnyList {
        let size = slice.len();
        let mut index: Vec<Option<Rc<SpinnyListCell>>> = vec![None; size];

        let mut current: Option<Rc<SpinnyListCell>> = None;
        let mut end_of_list: Option<Rc<SpinnyListCell>> = None;
        for &val in slice.iter().rev() {
            let cell = SpinnyListCell {
                val,
                tail: RefCell::new(current),
            };
            current = Some(Rc::new(cell));
            index[(val - 1) as usize] = current.clone();

            if end_of_list.is_none() {
                end_of_list = current.clone();
            }
        }
        let head = current.clone();
        let end = end_of_list.unwrap();

        *end.tail.borrow_mut() = current;
        SpinnyList { head, index }
    }

    fn move_next_chunk(&mut self, chunk_size: usize, dest_val: u32) {
        let head_cell = self.head.clone().unwrap();
        let startpoint = head_cell.next();
        let mut penultimate = startpoint.clone();
        let mut values_being_moved: Vec<u32> = vec![startpoint.val];

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
        let destination = self.find_cell(dest_val).unwrap();
        let destination_next = destination.next();

        let endpoint = penultimate.next();

        *head_cell.tail.borrow_mut() = Some(endpoint.clone());
        *destination.tail.borrow_mut() = Some(startpoint.clone());
        *penultimate.tail.borrow_mut() = Some(destination_next.clone());
    }

    pub fn find_cell(&self, val: u32) -> Option<Rc<SpinnyListCell>> {
        self.index[(val - 1) as usize].clone()
    }

    pub fn first(&self) -> u32 {
        self.head.as_ref().expect("list was empty").value()
    }

    pub fn jump_to(&mut self, val: u32) {
        self.head = self.find_cell(val);
    }

    pub fn step_to_next(&mut self) {
        self.head = Some(self.head.as_ref().unwrap().next())
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// "Unwind" the SpinnyList into a vector.
    pub fn to_vec(&mut self) -> Vec<u32> {
        let mut vals: Vec<u32> = Vec::new();
        let mut cell = self.head.clone().expect("Empty lists aren't supported");
        while vals.len() < self.len() {
            vals.push(cell.value());
            cell = cell.next();
        }
        vals
    }
}

fn perform_cup_move(cups: &mut SpinnyList) {
    let first_cup: u32 = cups.first();
    let destination: u32 = if first_cup == 1 {
        cups.len() as u32
    } else {
        first_cup - 1
    };
    cups.move_next_chunk(3, destination);
    cups.step_to_next();
}

fn order_after_n_moves(cups: &mut SpinnyList, count: u32) -> Vec<u32> {
    for iter in 0..count {
        if iter % 10000 == 0 {
            dbg!(iter);
        }
        perform_cup_move(cups);
    }
    cups.jump_to(1);
    let mut order = cups.to_vec();
    order.remove(0);
    order
}

fn main() {
    let mut cups: SpinnyList = SpinnyList::from_slice(&[5, 8, 9, 1, 7, 4, 2, 6, 3]);
    let order = order_after_n_moves(&mut cups, 100);
    println!("Cup order for part 1: {:?}", order);

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

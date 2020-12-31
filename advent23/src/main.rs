use std::cell::RefCell;
use std::collections::VecDeque;
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

    fn rest(&self) -> Option<Rc<Self>> {
        match *self.tail.borrow() {
            Some(ref tail) => Some(tail.clone()),
            None => None,
        }
    }

    fn insert_slice(&mut self, slice: &[u32]) {
        let mut current: Option<Rc<SpinnyListCell>> = self.tail.borrow().clone();
        for &val in slice.iter().rev() {
            let cell = SpinnyListCell {
                val,
                tail: RefCell::new(current),
            };
            current = Some(Rc::new(cell));
        }
        *self.tail.borrow_mut() = current;
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

        // let mut end_of_list: Option<Rc<RefCell<Option<SpinnyListCell>>>> = None;
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

    pub fn find_cell(&self, val: u32) -> Option<Rc<SpinnyListCell>> {
        self.index[(val - 1) as usize].clone()
    }

    pub fn first(&self) -> u32 {
        self.head.as_ref().expect("list was empty").value()
    }

    pub fn next(&mut self) -> u32 {
        let val = self.head.as_ref().unwrap().val;
        let nextref = self
            .head
            .as_ref()
            .expect("why is the SpinnyList empty")
            .rest();
        self.head = Some(nextref.expect("reached the end of the SpinnyList somehow"));
        val
    }

    pub fn jump_to(&mut self, val: u32) {
        self.head = self.find_cell(val);
    }

    pub fn len(&self) -> usize {
        self.index.len()
    }

    /// "Unwind" the SpinnyList into a vector.
    pub fn to_vec(&mut self) -> Vec<u32> {
        let mut vals: Vec<u32> = Vec::new();
        while vals.len() < self.len() {
            let next_val = self.next();
            vals.push(next_val);
        }
        vals
    }
}

fn perform_cup_move_new(cups: &mut SpinnyList) {
    let highest_cup = cups.len() as u32;
    let current_cup = cups.first();

    // cups.
}

fn perform_cup_move(cups: &mut VecDeque<u32>) {
    let highest_cup = cups.len() as u32;
    let current_cup = cups.pop_front().unwrap();
    let mut picked_up: Vec<u32> = Vec::new();
    for _i in 0..3 {
        picked_up.push(cups.pop_front().unwrap());
    }
    let mut target_cup = current_cup - 1;
    if target_cup == 0 {
        target_cup = highest_cup;
    }
    while picked_up.contains(&target_cup) {
        target_cup -= 1;
        if target_cup == 0 {
            target_cup = highest_cup;
        }
    }

    let target_index = cups.iter().position(|&x| x == target_cup).unwrap();
    for _i in 0..3 {
        cups.insert(target_index + 1, picked_up.pop().unwrap());
    }

    cups.push_back(current_cup);
}

fn order_after_n_moves(mut cups: VecDeque<u32>, count: usize) -> Vec<u32> {
    for iter in 0..count {
        if iter % 1000 == 0 {
            dbg!(iter);
        }
        perform_cup_move(&mut cups);
    }
    loop {
        let first_cup = cups.pop_front().unwrap();
        if first_cup == 1 {
            break;
        }
        cups.push_back(first_cup);
    }
    cups.iter().cloned().collect()
}

fn main() {
    let cups: VecDeque<u32> = VecDeque::from(vec![5, 8, 9, 1, 7, 4, 2, 6, 3]);
    let order = order_after_n_moves(cups, 100);
    println!("Cup order for part 1: {:?}", order);

    let mut cups: VecDeque<u32> = VecDeque::from(vec![5, 8, 9, 1, 7, 4, 2, 6, 3]);
    for cup in 10..=1_000_000 {
        cups.push_back(cup);
    }
    let order = order_after_n_moves(cups, 10_000_000);
    println!("Product for part 2: {}", order[0] * order[1]);
}

#[test]
fn test_individual_moves() {
    let mut cups: VecDeque<u32> = VecDeque::from(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
    perform_cup_move(&mut cups);
    assert_eq!(Vec::from(cups.clone()), vec![2, 8, 9, 1, 5, 4, 6, 7, 3]);
    perform_cup_move(&mut cups);
    assert_eq!(Vec::from(cups), vec![5, 4, 6, 7, 8, 9, 1, 3, 2]);
}

#[test]
fn test_100_moves() {
    let cups: VecDeque<u32> = VecDeque::from(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
    let answer = order_after_n_moves(cups, 100);
    assert_eq!(answer, vec![6, 7, 3, 8, 4, 5, 2, 9]);
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

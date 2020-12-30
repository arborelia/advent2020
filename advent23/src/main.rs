use std::collections::VecDeque;

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

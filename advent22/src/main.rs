use eyre::Result;
use std::collections::VecDeque;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Player {
    P1,
    P2,
}

fn play_one_turn(
    p1: &mut VecDeque<u32>,
    p2: &mut VecDeque<u32>,
    recursive: bool,
    known_games: &mut HashMap<String, Player>,
) {
    let card1 = p1.pop_front().unwrap();
    let card2 = p2.pop_front().unwrap();
    if recursive && card1 as usize <= p1.len() && card2 as usize <= p2.len() {
        let num1 = card1 as usize;
        let num2 = card2 as usize;
        let p1_next: &[u32] = &p1.make_contiguous()[0..num1];
        let p2_next: &[u32] = &p2.make_contiguous()[0..num2];
        let mut recursive_deck1: VecDeque<u32> = p1_next.iter().copied().collect();
        let mut recursive_deck2: VecDeque<u32> = p2_next.iter().copied().collect();
        let winner = play_full_game(
            &mut recursive_deck1,
            &mut recursive_deck2,
            true,
            known_games,
        );
        match winner {
            Player::P1 => {
                p1.push_back(card1);
                p1.push_back(card2);
            }
            Player::P2 => {
                p2.push_back(card2);
                p2.push_back(card1);
            }
        }
    } else {
        if card1 > card2 {
            p1.push_back(card1);
            p1.push_back(card2);
        } else {
            p2.push_back(card2);
            p2.push_back(card1);
        }
    }
}

fn describe_state(p1: &VecDeque<u32>, p2: &VecDeque<u32>) -> String {
    format!("{:?}/{:?}", p1, p2)
}

fn play_full_game(
    p1: &mut VecDeque<u32>,
    p2: &mut VecDeque<u32>,
    recursive: bool,
    known_games: &mut HashMap<String, Player>,
) -> Player {
    let description = describe_state(p1, p2);
    if known_games.contains_key(&description) {
        return known_games[&description];
    }
    println!("{} known games. Playing {}", known_games.len(), description);

    let mut seen_states: HashSet<String> = HashSet::new();
    loop {
        let description = describe_state(p1, p2);
        if seen_states.contains(&description) {
            // the game would loop infinitely, and is a win for P1
            known_games.insert(description, Player::P1);
            return Player::P1;
        }
        seen_states.insert(description.clone());
        if p1.is_empty() {
            known_games.insert(description, Player::P2);
            return Player::P2;
        } else if p2.is_empty() {
            known_games.insert(description, Player::P1);
            return Player::P1;
        }
        play_one_turn(p1, p2, recursive, known_games);
    }
}

// adapted from Rust By Example
// this is honestly overkill for just getting the lines of the file, I just wanted
// to try doing things the "proper" way
fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn score_deck(deck: &VecDeque<u32>) -> u64 {
    let n_cards = deck.len();
    deck.iter()
        .enumerate()
        .map(|(pos, &val)| val as u64 * (n_cards - pos) as u64)
        .sum()
}

fn main() -> Result<()> {
    let lines = read_lines("input.txt")?;
    let mut state: Player = Player::P1;
    let mut deck1: VecDeque<u32> = VecDeque::new();
    let mut deck2: VecDeque<u32> = VecDeque::new();
    let mut known_games: HashMap<String, Player> = HashMap::new();

    for line in lines {
        let line = line?;
        if line != "" {
            if line == "Player 1:" {
                state = Player::P1;
            } else if line == "Player 2:" {
                state = Player::P2;
            } else {
                let value: u32 = line.parse()?;
                match state {
                    Player::P1 => deck1.push_back(value),
                    Player::P2 => deck2.push_back(value),
                }
            }
        }
    }
    let mut deck1r = deck1.clone();
    let mut deck2r = deck2.clone();
    let winner = play_full_game(&mut deck1, &mut deck2, false, &mut known_games);
    let winning_deck = match winner {
        Player::P1 => deck1,
        Player::P2 => deck2,
    };

    println!("without recursion: {:?}", winning_deck);
    let winning_score: u64 = score_deck(&winning_deck);
    println!("score: {}", winning_score);

    let winner = play_full_game(&mut deck1r, &mut deck2r, true, &mut known_games);
    let winning_deck = match winner {
        Player::P1 => deck1r,
        Player::P2 => deck2r,
    };

    println!("with recursion: {:?}", winning_deck);
    let winning_score: u64 = score_deck(&winning_deck);
    println!("score: {}", winning_score);

    Ok(())
}

#[test]
fn test_nonrecursive() {
    let mut p1: VecDeque<u32> = VecDeque::from(vec![9, 2, 6, 3, 1]);
    let mut p2: VecDeque<u32> = VecDeque::from(vec![5, 8, 4, 7, 10]);
    let mut known_games: HashMap<String, Player> = HashMap::new();
    let winner = play_full_game(&mut p1, &mut p2, false, &mut known_games);
    assert_eq!(winner, Player::P2);
    assert_eq!(score_deck(&p2), 306);
}

#[test]
fn test_recursive_that_loops() {
    let mut p1: VecDeque<u32> = VecDeque::from(vec![43, 19]);
    let mut p2: VecDeque<u32> = VecDeque::from(vec![2, 29, 14]);
    let mut known_games: HashMap<String, Player> = HashMap::new();
    let winner = play_full_game(&mut p1, &mut p2, true, &mut known_games);
    assert_eq!(winner, Player::P1);
    assert_eq!(score_deck(&p1), 43 * 2 + 19);
}

#[test]
fn test_recursive() {
    let mut p1: VecDeque<u32> = VecDeque::from(vec![9, 2, 6, 3, 1]);
    let mut p2: VecDeque<u32> = VecDeque::from(vec![5, 8, 4, 7, 10]);
    let mut known_games: HashMap<String, Player> = HashMap::new();
    let winner = play_full_game(&mut p1, &mut p2, true, &mut known_games);
    assert_eq!(winner, Player::P2);
    assert_eq!(score_deck(&p2), 291);
}

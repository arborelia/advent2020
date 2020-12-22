use std::collections::HashMap;

fn elf_sequence(init: &[usize], steps: usize) -> usize {
    // Keep track of when each given integer last occurred.
    let mut last_spoken: HashMap<usize, usize> = HashMap::new();
    let mut current: usize = 0;

    for step in 0..(steps - 1) {
        // if we're in the initial steps, override 'current' with the provided number
        // from the init sequence
        if step < init.len() {
            current = init[step];
        }

        // Update the "last spoken" time for the current number to the current time step.
        // This returns an Option of the old value, if any.
        let maybe_last_time: Option<usize> = last_spoken.insert(current, step);
        if let Some(last_time) = maybe_last_time {
            // set "current" to the elapsed time since that number was last spoken
            current = step - last_time;
        } else {
            current = 0;
        }
    }
    current
}

fn main() {
    let input: Vec<usize> = vec![16, 11, 15, 0, 1, 7];
    let result = elf_sequence(&input, 2020);
    println!("the 2020th number spoken was {}", result);

    let result = elf_sequence(&input, 30_000_000);
    println!("the 30 millionth number spoken was {}", result);
}

#[test]
fn test_elf_sequence() {
    assert_eq!(elf_sequence(&[0, 3, 6], 10), 0);
    assert_eq!(elf_sequence(&[1, 3, 2], 2020), 1);
    assert_eq!(elf_sequence(&[2, 1, 3], 2020), 10);
    assert_eq!(elf_sequence(&[3, 1, 2], 2020), 1836);
    assert_eq!(elf_sequence(&[0, 3, 6], 2020), 436);
}

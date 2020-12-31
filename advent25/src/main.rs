use mod_exp::mod_exp;

const INITIAL: u64 = 7;
const MODULUS: u64 = 20201227;

/// Solve the problem (base ^ n) % modulus = exponentiated, for n.
fn discrete_log(base: u64, modulus: u64, exponentiated: u64) -> u64 {
    let mut product: u64 = 1;
    for exponent in 0..modulus {
        if product == exponentiated {
            return exponent;
        }
        product = (product * base) % modulus;
    }
    panic!("math broke");
}

fn break_encryption(public_key1: u64, public_key2: u64) -> u64 {
    let private_key1 = discrete_log(INITIAL, MODULUS, public_key1);
    let private_key2 = discrete_log(INITIAL, MODULUS, public_key2);
    let handshake1 = mod_exp(public_key1, private_key2, MODULUS);
    let handshake2 = mod_exp(public_key2, private_key1, MODULUS);
    assert_eq!(handshake1, handshake2);
    handshake1
}

fn main() {
    let (key1, key2) = (12090988, 240583);
    let handshake = break_encryption(key1, key2);
    println!("Handshake value: {}", handshake);
}

#[test]
fn test_discrete_log() {
    assert_eq!(discrete_log(7, MODULUS, 5764801), 8);
    assert_eq!(discrete_log(7, MODULUS, 17807724), 11);
}

#[test]
fn test_handshake() {
    assert_eq!(mod_exp(17807724, discrete_log(7, MODULUS, 5764801), MODULUS), 14897079);
    assert_eq!(mod_exp(5764801, discrete_log(7, MODULUS, 17807724), MODULUS), 14897079);
}
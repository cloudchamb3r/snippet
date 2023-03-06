#![allow(unused_variables, dead_code)]
use std::char;

pub fn luhn(cc_number: &str) -> bool {
    let mut cc_number_vec: Vec<char> = cc_number
        .chars()
        .filter(|c| !c.is_whitespace() && c.ne(&'-'))
        .collect();

    if cc_number_vec.iter().filter(|c| !c.is_numeric()).count() != 0 || cc_number_vec.len() <= 2 {
        return false;
    }

    cc_number_vec
        .iter_mut()
        .rev()
        .skip(1)
        .step_by(2)
        .for_each(|c| {
            if let Some(d) = c.to_digit(10) {
                let d = d * 2;
                if d >= 10 {
                    *c = char::from_digit((d % 10) + (d / 10), 10).unwrap();
                } else {
                    *c = char::from_digit(d, 10).unwrap();
                }
            }
        });

    let sum: u32 = cc_number_vec.iter().map(|c| c.to_digit(10).unwrap()).sum();
    sum % 10 == 0
}

#[test]
fn test_non_digit_cc_number() {
    assert!(!luhn("foo"));
}

#[test]
fn test_empty_cc_number() {
    assert!(!luhn(""));
    assert!(!luhn(" "));
    assert!(!luhn("   "));
    assert!(!luhn("    "));
}

#[test]
fn test_single_digit_cc_number() {
    assert!(!luhn("0"));
}

#[test]
fn test_two_digit_cc_number() {
    assert!(!luhn(" 0 0 "));
}

#[test]
fn test_valid_cc_number() {
    assert!(luhn("4263 9826 4026 9299"));
    assert!(luhn("4539 3195 0343 6467"));
    assert!(luhn("7992 7398 713"));
}

#[test]
fn test_invalid_cc_number() {
    assert!(!luhn("4223 9826 4026 9299"));
    assert!(!luhn("4539 3195 0343 6476"));
    assert!(!luhn("8273 1232 7352 0569"));
}

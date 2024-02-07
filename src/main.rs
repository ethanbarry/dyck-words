#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use clap::Parser;
use itertools::*;
use std::io::Write;

#[derive(Parser)]
struct Cli {
    semilength: u64,
}

#[derive(Debug)]
enum ParseError {
    InvalidDigit(char),
}

fn main() {
    let semilength = Cli::parse().semilength;
    let num_dyck_words = catalan_number(semilength) - 1; // We cover the first case with the above fn call.
                                                         // Thus this is the number of REMAINING Dyck words.

    // Lock stdout to get a bit of a speed improvement with large numbers of writes.
    let stdout = std::io::stdout();
    let mut lock = stdout.lock();

    writeln!(
        lock,
        "\nThe number of Dyck words of semilength {} is: {}\n",
        semilength,
        num_dyck_words + 1
    )
    .unwrap();

    // TODO Construct the first binary string of pattern ()()()(), or 10101010...
    let mut binary_string = minimal_dyck_word(semilength);
    writeln!(lock, "{binary_string}").unwrap();

    // For all the remaining Dyck words...
    for _ in 0..num_dyck_words {
        match parse_binary_string(&binary_string) {
            Ok(value) => {
                // Compute and print the binary representation of the word.
                binary_string = format!("{:#0b}", next_dyck_word_bin(value)).to_owned();
                writeln!(lock, "{binary_string}").unwrap();
            }
            Err(error) => {
                writeln!(lock, "Error: {error:?}").unwrap();
            }
        }
    }
}

/// Compute the Dyck word of semilength n and form ()()()(), or 10101010.
fn minimal_dyck_word(n: u64) -> String {
    let zeros = vec!['0'; n.try_into().unwrap()];
    let ones = vec!['1'; n.try_into().unwrap()];

    let bin: String = ones.into_iter().interleave(zeros).collect();
    "0b".to_owned() + &bin
}

/// Parse a string like "0b10101010" to a [`u64`].
fn parse_binary_string(binary_string: &str) -> Result<u64, ParseError> {
    let mut result = 0_u64;
    for c in binary_string.chars().skip(2) {
        match c {
            '0' => result <<= 1,
            '1' => result = (result << 1) | 1,
            _ => return Err(ParseError::InvalidDigit(c)),
        }
    }
    Ok(result)
}

/// Compute the nth Catalan number. Called once.
fn catalan_number(n: u64) -> u64 {
    // Edge cases for base values.
    if n == 0 || n == 1 {
        return 1;
    }

    // Initialize an array to store Catalan numbers up to n.
    let mut catalan_numbers = vec![0_u64; (n + 1) as usize];
    catalan_numbers[0] = 1;
    catalan_numbers[1] = 1;

    // Efficiently calculate Catalan numbers using dynamic programming.
    for i in 2..=n as usize {
        for j in 0..i {
            catalan_numbers[i] += catalan_numbers[j] * catalan_numbers[i - j - 1];
        }
    }

    // Return the calculated nth Catalan number.
    catalan_numbers[n as usize]
}

/// Here's where the magic happens. By manipulating bytes, we can represent Dyck
/// words as bits, which lets us do fast math on them. This algorithm returns
/// the 'next' Dyck word, without checking whether it is valid or not.
/// Therefore, to use it, we iterate over the number of possible Dyck words of
/// semilength n, which is C_n, the Catalan number of order n.
/// This algorithm must start with the 'least' Dyck word, or lowest-valued
/// integer.
fn next_dyck_word_bin(w: u64) -> u128 {
    // Calculate a, potentially using two's complement for negation.
    let a = w & (!w + 1);

    // Calculate b directly.
    let b = w + a;

    // Calculate c using bitwise XOR.
    let c = w ^ b;

    // Shift c right by 2, ensuring unsigned division.
    let c = (c as u128 / a as u128) >> 2; // Cast to u128 for division.

    // Add 1 to c, handling potential overflow.
    let c = c.wrapping_add(1);

    // Apply mask and bitwise OR with b.
    let mask = 0xaaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa_aaaa; // I think this is long enough...

    // Return the resulting Dyck word as a u128.
    ((c * c - 1) & mask) | b as u128
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn check_catalan_numbers() {
        let c_5 = catalan_number(5);
        assert_eq!(c_5, 42);
    }
}

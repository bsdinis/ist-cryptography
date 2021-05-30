use paillier_common::*;
use polynomials::poly;
use ppso::{element::Element, rand_poly};
use std::{io, io::Result as ioResult};

fn parse_elements() -> ioResult<Vec<Element>> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer)?;
    let buffer = buffer.trim();
    Ok(buffer
        .split(',')
        .map(|string| string.parse().unwrap())
        .collect())
}

fn main() -> ioResult<()> {
    println!("Choose 3 elements for player 1 (comma separated integers)");
    let player1_elements = parse_elements()?;
    let f1 = {
        player1_elements
            .iter()
            .map(|element| poly![1, -(element.value() as i64)])
            .fold(poly![1], |acc, curr| acc * curr)
    };

    println!("Choose 3 elements for player 2 (comma separated integers)");
    let player2_elements = parse_elements()?;
    let f2 = {
        player2_elements
            .iter()
            .map(|element| poly![1, -(element.value() as i64)])
            .fold(poly![1], |acc, curr| acc * curr)
    };

    let (ek, dk) = Paillier::keypair().keys();

    let r12 = rand_poly(3);
    let r21 = rand_poly(3);

    Ok(())
}

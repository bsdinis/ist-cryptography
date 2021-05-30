use polynomials::{poly, Polynomial};
use rand::Rng;

pub mod element;

pub fn rand_poly(degree: usize) -> Polynomial<i64> {
    let mut rng = rand::thread_rng();
    (0..degree)
        .map(|_| {
            let rand = rng.gen();
            poly![1, rand]
        })
        .fold(poly![1], |acc, curr| acc * curr)
}

#[cfg(test)]
mod tests;

use paillier_common::*;
use polynomials::{poly, Polynomial};

#[test]
#[ignore]
fn ppso_works() {
    // 1
    // a
    // b
    // c
    // d

    // 2

    // 3
    // a
    // b
    // c

    // 4

    // 5

    todo!()
}

#[test]
fn polynomials_works() {
    let a = poly![1, -2];
    let b = poly![1, 2];
    let c = a * b;
    let v: Vec<_> = c.into();
    dbg!(&v);
    let p: Polynomial<_> = v.into();
    dbg!(p);
}

#[test]
fn paillier_works() {
    // generate a fresh keypair and extract encryption and decryption keys
    let (ek, dk) = Paillier::keypair().keys();

    // encrypt four values
    let c1 = Paillier::encrypt(&ek, 10);
    let c2 = Paillier::encrypt(&ek, 20);
    let c3 = Paillier::encrypt(&ek, 30);
    let c4 = Paillier::encrypt(&ek, 40);

    // add all of them together
    let c = Paillier::add(
        &ek,
        &Paillier::add(&ek, &c1, &c2),
        &Paillier::add(&ek, &c3, &c4),
    );

    // multiply the sum by 2
    let d = Paillier::mul(&ek, &c, 2);

    // decrypt final result
    let m: u64 = Paillier::decrypt(&dk, &d);
    println!("decrypted total sum is {}", m);
}

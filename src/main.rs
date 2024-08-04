mod numbers;
use numbers::IntToBigUint;

fn main() {
    let one: num::bigint::BigUint = 1.biguint();

    let p = numbers::gen_prime();

    let q = numbers::gen_prime();

    let n = &p * &q;
    println!("n with {} bits", n.bits());

    let phi = (&p - &one) * (&q - &one);

    let e = numbers::coprime(&phi);
    println!("e with {} bits", e.bits());

    let d = &e.modinv(&phi).unwrap();
    println!("d with {} bits", d.bits());

    let m = numbers::random(1 << 10);

    let c = numbers::pow_mod(m.clone(), e.clone(), &n);
    let pt = numbers::pow_mod(c.clone(), d.clone(), &n);

    assert!(m == pt);
    println!("RSA OK");
}

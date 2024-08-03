use num::{self, bigint::RandBigInt, BigUint, FromPrimitive, Integer};

pub const BIT_SIZE: u64 = 1024;
pub const MILLER_ROUNDS: usize = 0xF00;

fn main() {
    let one = BigUint::from_u8(1).unwrap();

    let p = gen_prime();

    let q = gen_prime();

    let n = &p * &q;
    println!("n with {} bits", n.bits());

    let phi = (&p - &one) * (&q - &one);

    let e = coprime(&phi);
    println!("e with {} bits", e.bits());

    let d = &e.modinv(&phi).unwrap();
    println!("d with {} bits", d.bits());

    let mut rng = rand::thread_rng();
    let m = rng.gen_biguint(1 << 10);

    let c = pow_mod(m.clone(), e.clone(), &n);
    let pt = pow_mod(c.clone(), d.clone(), &n);

    assert!(m == pt);
    println!("RSA OK");
}

/// computes pow(base, exp) % m
fn pow_mod(mut base: BigUint, mut exp: BigUint, m: &BigUint) -> BigUint {
    let mut result: BigUint = BigUint::from_u8(1).unwrap();

    base %= m;

    let one = BigUint::from_u8(1).unwrap();
    while &exp > &BigUint::ZERO {
        if &exp & &one == one {
            result *= &base;
            result %= m;
            exp -= BigUint::from_u8(1).unwrap();
        } else {
            exp >>= 1;
            base = base.pow(2);
            base %= m;
        }
    }

    return result;
}

fn gen_prime() -> BigUint {
    let mut rng = rand::thread_rng();
    let mut prime = rng.gen_biguint(BIT_SIZE);
    loop {
        if miller::is_prime(&prime, MILLER_ROUNDS) {
            return prime;
        }
        prime = rng.gen_biguint(BIT_SIZE);
    }
}

fn coprime(n: &BigUint) -> BigUint {
    let one = BigUint::from_u8(1).unwrap();
    let mut e = n.clone() - &one - &one;

    while &e > &one {
        if &e.gcd(n) == &one {
            return e;
        }
        e -= &one;
    }

    return e;
}

mod miller {

    use super::*;

    pub fn is_prime(n: &BigUint, rounds: usize) -> bool {
        let two: BigUint = BigUint::from_u8(2).unwrap();
        let three: BigUint = BigUint::from_u8(3).unwrap();
        let five: BigUint = BigUint::from_u8(5).unwrap();

        if n == &two || n == &three || n == &five {
            return true;
        }

        let one: BigUint = BigUint::from_u8(1).unwrap();

        if n & &one == BigUint::ZERO {
            // n is even -> not a prime
            return false;
        }

        let mut rng = rand::thread_rng();

        let d = compute_d(&n);
        let a = rng.gen_biguint(BIT_SIZE);

        let mut x = pow_mod(a, d, &n);
        if &x == &one || &x == &(n - &one) {
            return true;
        }

        for _ in 0..rounds {
            x = pow_mod(x.clone(), two.clone(), n);
            if &x == &one {
                return false;
            }

            if &x == &(n - &one) {
                return true;
            }
        }

        return false;
    }

    fn compute_d(n: &BigUint) -> BigUint {
        let one: BigUint = BigUint::from_u8(1).unwrap();
        let two: BigUint = BigUint::from_u8(2).unwrap();

        let mut d = n - &one;
        while &d & &one == BigUint::ZERO {
            d /= &two;
        }

        return d;
    }
}

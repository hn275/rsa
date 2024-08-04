use num::{bigint::RandBigInt, BigUint, FromPrimitive, Integer};
use rand;

pub const BIT_SIZE: u64 = 1024;
pub const MILLER_ROUNDS: usize = 0xF00;

pub fn random(bit_size: u64) -> BigUint {
    let mut rng = rand::thread_rng();
    rng.gen_biguint(bit_size)
}

pub trait IntToBigUint {
    fn biguint(self) -> BigUint;
}

impl IntToBigUint for u32 {
    fn biguint(self) -> BigUint {
        BigUint::from_u32(self).unwrap()
    }
}

/// computes pow(base, exp) % m
pub fn pow_mod(mut base: BigUint, mut exp: BigUint, m: &BigUint) -> BigUint {
    let mut result: BigUint = 1.biguint();

    base %= m;

    let one = 1.biguint();
    while &exp > &BigUint::ZERO {
        if &exp & &one == one {
            result *= &base;
            result %= m;
            exp -= &one
        } else {
            exp >>= 1;
            base = base.pow(2);
            base %= m;
        }
    }

    return result;
}

pub fn gen_prime() -> BigUint {
    let mut rng = rand::thread_rng();
    let mut prime = rng.gen_biguint(BIT_SIZE);
    loop {
        if miller::is_prime(&prime, MILLER_ROUNDS) {
            return prime;
        }
        prime = rng.gen_biguint(BIT_SIZE);
    }
}

pub fn coprime(n: &BigUint) -> BigUint {
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

pub mod miller {

    use super::*;

    pub fn is_prime(n: &BigUint, rounds: usize) -> bool {
        let two = 2.biguint();
        let three = 3.biguint();
        let five = 5.biguint();

        if n == &two || n == &three || n == &five {
            return true;
        }

        let one = 1.biguint();

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
        let one = 1.biguint();

        let mut d = n - &one;
        while &d & &one == BigUint::ZERO {
            d >>= 1;
        }

        return d;
    }
}

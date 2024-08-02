import random
import math
from typing import Optional

MILLER_ROUNDS = 0x1000


# power with mod, increase of overflow
# return (base ** exp) % mod
def pow(base: int, exp: int, mod: int) -> int:

    result = 1
    base %= mod

    while exp > 0:
        if exp & 1 == 1:  # if exp is odd
            result = (result * base) % mod
            exp -= 1

        else:
            # a**b % p
            # = a**(2 * b/2) % p
            # = (a**2)**(b/2) % p
            # = (a*a)**(b/2) % p <- reduce base `a` by `mod p` again
            exp >>= 1
            base = (base * base) % mod

    return result


# compute `d` and `r` s.t.:
# n - 1 = d * (2**r), r >= 1
# return (d, r)
# n is odd and n != 1
def compute_d(n: int) -> int:
    assert n & 1 == 1 and n > 1

    d = n - 1
    while d & 1 == 0:
        d //= 2

    return d


# testing if `n` is prime.
# greater round = more accurate test
def millertest(n: int, round: int) -> bool:
    # `n` needs to be greater than 5,
    # since `random.randint(2, n - 2)` will panic if 2 >= n-2
    if n in [2, 3, 5]:
        return True

    if n & 1 == 0:  # n is even -> not prime
        return False

    d = compute_d(n)

    a = random.randint(2, n - 2)
    x = pow(a, d, n)
    round -= 1

    if x == 1 or x == n - 1:
        return True

    while round >= 0:
        x = pow(x, 2, n)

        if x == 1:
            break

        if x == n - 1:
            return True

        round -= 1

    return False


# generate a random prime
def genprime() -> int:
    p: int = random.randint(0x100000, 0xFFFFFF)
    while millertest(p, MILLER_ROUNDS) == False:
        p = random.randint(0x100000, 0xFFFFFF)

    return p


# calculate coprime of n
def coprime(n: int) -> int:
    e = n - 2
    while e > 1:
        if math.gcd(e, n) == 1:
            return e
        e -= 1
    return e


# extended euclidean algorithm, used to find mod inverse
# return gcd, x, y
# if gcd != 1, mod inverse does not exist
def extended_gcd(a: int, b: int) -> tuple[int, int, int]:
    if a == 0:
        return b, 0, 1

    gcd, x1, y1 = extended_gcd(b % a, a)
    x = y1 - (b // a) * x1
    y = x1
    return gcd, x, y


# multiplicative modulo inverse of `a mod m` (if exists)
def mod_inverse(a, m) -> Optional[int]:
    gcd, x, _ = extended_gcd(a, m)
    if gcd != 1:
        return None
    return x % m


def print_int(pre: str, n: int) -> None:
    # print(f"{pre}: [dec]{n}, [bin]{n:#b}")
    print(f"{pre}: {n}")


if __name__ == "__main__":
    p = genprime()
    print(f"p: {p}")
    q = genprime()
    print(f"q: {q}")

    # no twin-primes, easy to crack `n` with twin-primes
    assert p != q and abs(p - q) != 2

    n = p * q
    print_int("n", n)

    # euler's function, from the original RSA
    phi = (p - 1) * (q - 1)
    e = coprime(phi)
    print_int("e", e)

    d = mod_inverse(e, phi)
    assert d != None  # if `e` and `phi` are computed properly, this won't fail
    print_int("d", d)

    # RSA pki:
    # params: `e` and `n` are public, everything else is private
    # C - ciphertext
    # M - message/plaintext

    # encrypt: C congru. (M**e) % n
    m = random.randint(0, 0xFFFFFF)
    print_int("M", m)

    c = pow(m, e, n)
    print_int("C", c)

    # decryption: M congru. (C**d) % n
    pt = pow(c, d, n)
    print_int("pt", pt)

    if m == pt:
        print("RSA ok")
    else:
        print("wrong :/")

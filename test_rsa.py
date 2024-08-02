import random
from .rsa import coprime, genprime, millertest, mod_inverse
import math


def test_millertest():
    primes = [
        2,
        3,
        5,
        7,
        11,
        13,
        17,
        19,
        23,
        29,
        31,
        37,
        41,
        43,
        47,
        53,
        59,
        61,
        67,
        71,
        73,
        79,
        83,
        89,
        97,
    ]

    for prime in primes:
        assert millertest(prime, 100)


def test_gen_prime():
    for _ in range(100):
        assert millertest(genprime(), 100)


def test_coprime():
    for _ in range(100):
        i = random.randint(5, 1000000)
        assert math.gcd(coprime(i), i) == 1


def test_mod_inverse():
    m = random.randint(5, 10000)
    for _ in range(100):
        x = random.randint(5, 10000)
        try:
            assert (mod_inverse(x, m) * x) % m == 1
        except ValueError:
            # some values don't have mod inverse, so long as the assertion pass,
            # that's okay
            continue

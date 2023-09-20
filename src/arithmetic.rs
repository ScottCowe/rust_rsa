pub fn modulus(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

pub fn greatest_common_divisor(a: i64, b: i64) -> i64 {
    let modulus: i64 = modulus(a, b);

    if modulus == 0 {
        return b;
    }

    greatest_common_divisor(b, modulus)
}

pub fn lowest_common_multiple(a: i64, b: i64) -> i64 {
    let gcd: i64 = greatest_common_divisor(a, b);

    (a * b) / gcd
}

// Solves for x and y given that: ax + by = gcd(a, b)
pub fn extended_euclidian(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return ((a as u64).try_into().unwrap(), 1, 0);
    }

    let mut quotient: i64 = a / b;
    let mut remainder: i64 = a % b;
    let mut a1: i64 = 1;
    let mut a2: i64 = 0;
    let mut a3: i64 = a1 - quotient * a2;
    let mut b1: i64 = 0;
    let mut b2: i64 = 1;
    let mut b3: i64 = b1 - quotient * b2;

    while remainder != 0 {
        a = b;
        b = remainder;
        quotient = a / b;
        remainder = a % b;
        a1 = a2;
        a2 = a3;
        a3 = a1 - quotient * a2;
        b1 = b2;
        b2 = b3;
        b3 = b1 - quotient * b2;
    }

    (b, a2, b2)
}

pub fn multiplicitive_inverse(a: i64, n: i64) -> u64 {
    let (gcd, a, b): (i64, i64, i64) = extended_euclidian(n, a);

    if gcd != 1 {
        panic!("b and n are not co-primes");
    }

    modulus(b, n) as u64
}

// Calculates the result of a^b mod c
// https://rustp.org/number-theory/modular-exponentiation/
// Inefficent algorithm used for testing purposes
pub fn modular_exponent(a: u64, b: u64, c: u64) -> u64 {
    let mut answer: u64 = 1;

    for _ in 0..b {
        answer *= a;
        answer %= c;
    }

    return answer;
}


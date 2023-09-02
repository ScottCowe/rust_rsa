use rand::Rng;
use std::ops::Rem;

fn main() {
    let primes: Vec<u32> = find_primes_smaller_than(10000);

    let prime1: &u32 = &primes[rand::thread_rng().gen_range(0..=primes.len())];
    let prime2: &u32 = &primes[rand::thread_rng().gen_range(0..=primes.len())];

    let primes_multiplied: u32 = prime1 * prime2;

    let lcm: u32 = lowest_common_multiple(prime1 - 1, prime2 - 1);

    let exponent: u32 = 65537;

    let mult_inv: i32 = multiplicitive_inverse(17, 3120);
    println!("{}", mult_inv);
}

// Uses Sieve of Eratosthenes algorithm
fn find_primes_smaller_than(n: u32) -> Vec<u32> {
    let mut bool_results: Vec<bool> = vec![true; n as usize + 1];
    let root_n: u32 = (n as f32).sqrt() as u32;

    let mut i: u32 = 2;

    while i <= root_n {
        let mut j: u32 = i * i;

        while j <= n {
            bool_results[j as usize] = false;
            j += i;
        }

        i += 1;
    }

    let mut results: Vec<u32> = Vec::new();
    let mut result_index: u32 = 0;

    for result in bool_results {
        if result == true && result_index > 1 {
            results.push(result_index);
        }

        result_index += 1;
    }

    results
}

fn greatest_common_divisor(a: u32, b: u32) -> u32 {
    let modulus: u32 = a % b;

    if modulus == 0 {
        return b;
    }

    greatest_common_divisor(b, modulus)
}

fn lowest_common_multiple(a: u32, b: u32) -> u32 {
    let gcd: u32 = greatest_common_divisor(a, b);

    (a * b) / gcd
}

// Solves for x and y given that: ax + by = gcd(a, b)
fn extended_euclidian(mut a: i32, mut b: i32) -> (i32, i32, i32) {
    if b == 0 {
        return ((a as u32).try_into().unwrap(), 1, 0);
    }

    let mut quotient: i32 = a / b;
    let mut remainder: i32 = a % b;
    let mut a1: i32 = 1;
    let mut a2: i32 = 0;
    let mut a3: i32 = a1 - quotient * a2;
    let mut b1: i32 = 0;
    let mut b2: i32 = 1;
    let mut b3: i32 = b1 - quotient * b2;

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

fn modulus(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

fn multiplicitive_inverse(a: i32, n: i32) -> i32 {
    let (b, a2, b2): (i32, i32, i32) = extended_euclidian(n, a);

    if b != 1 {
        panic!("b and n are not co-primes");
    }

    modulus(b2, n)
}

use rand::Rng;
use std::io;

fn main() {
    let primes: Vec<u64> = find_primes_smaller_than(1000);

    let prime1: u64 = 61; //&primes[rand::thread_rng().gen_range(0..=primes.len())];
    let prime2: u64 = 53; //&primes[rand::thread_rng().gen_range(0..=primes.len())];

    let n: u64 = prime1 * prime2;

    let carmichaels_totient: u64 = lowest_common_multiple(prime1 - 1, prime2 - 1);

    let exponent: u64 = 17; // Artificially small for testing purposes

    let mult_inv: i64 = multiplicitive_inverse(exponent as i64, carmichaels_totient as i64);

    println!("Primes are {} and {}", prime1, prime2);
    println!("Primes multiplied (n) is {}", n);
    println!("λ(n) is {}", carmichaels_totient);
    println!("Exponent is {}", exponent);
    println!("Multiplicitve inverse is {}", mult_inv);

    println!("Please enter number you wish to be encrypted:");
    let mut to_encrypt_string: String = String::new();

    io::stdin()
        .read_line(&mut to_encrypt_string)
        .expect("Failed to read line");

    let to_encrypt: u64 = to_encrypt_string.trim().parse().unwrap();
    let encrypted: i64 = encrypt(to_encrypt, carmichaels_totient, exponent);
    let decrypted: i64 = decrypt(encrypted as u64, mult_inv as u64, exponent);

    println!("Encrypted is {}", encrypted);
    println!("Decrypted is {}", decrypted);
}

fn encrypt(to_encrypt: u64, n: u64, e: u64) -> i64 {
    let result: i64 = multiplicitive_inverse(i64::pow(to_encrypt as i64, e as u32), n as i64);
    result
}

fn decrypt(to_decrypt: u64, n: u64, d: u64) -> i64 {
    let result: i64 = multiplicitive_inverse(i64::pow(to_decrypt as i64, d as u32), n as i64);
    result
}

// Uses Sieve of Eratosthenes algorithm
fn find_primes_smaller_than(n: u64) -> Vec<u64> {
    let mut bool_results: Vec<bool> = vec![true; n as usize + 1];
    let root_n: u64 = (n as f64).sqrt() as u64;

    let mut i: u64 = 2;

    while i <= root_n {
        let mut j: u64 = i * i;

        while j <= n {
            bool_results[j as usize] = false;
            j += i;
        }

        i += 1;
    }

    let mut results: Vec<u64> = Vec::new();
    let mut result_index: u64 = 0;

    for result in bool_results {
        if result == true && result_index > 1 {
            results.push(result_index);
        }

        result_index += 1;
    }

    results
}

fn greatest_common_divisor(a: u64, b: u64) -> u64 {
    let modulus: u64 = a % b;

    if modulus == 0 {
        return b;
    }

    greatest_common_divisor(b, modulus)
}

fn lowest_common_multiple(a: u64, b: u64) -> u64 {
    let gcd: u64 = greatest_common_divisor(a, b);

    (a * b) / gcd
}

// Solves for x and y given that: ax + by = gcd(a, b)
fn extended_euclidian(mut a: i64, mut b: i64) -> (i64, i64, i64) {
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

fn modulus(a: i64, b: i64) -> i64 {
    ((a % b) + b) % b
}

fn multiplicitive_inverse(a: i64, n: i64) -> i64 {
    let (b, a2, b2): (i64, i64, i64) = extended_euclidian(n, a);

    if b != 1 {
        panic!("b and n are not co-primes");
    }

    modulus(b2, n)
}

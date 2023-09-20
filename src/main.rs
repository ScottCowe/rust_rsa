use rand::Rng;
use std::io;

pub mod arithmetic;
pub mod key;
pub mod primes;

fn main() {
    let primes: Vec<u64> = crate::primes::find_primes_smaller_than(1000);
}

/*fn main() {
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
}*/

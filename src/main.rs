use crate::arithmetic::*;
use crate::key::PrivateKey;
use crate::key::PublicKey;
use crate::primes::find_primes_smaller_than;
use rand::Rng;
use std::io;

pub mod arithmetic;
pub mod key;
pub mod primes;

fn main() {
    let prime_numbers: Vec<u64> = find_primes_smaller_than(1000);
    let (public_key, private_key): (PublicKey, PrivateKey) = generate_keypair(prime_numbers);

    let message: u64 = 3234; // Loops back around at 3234. Very strange

    let encrypted: u64 = public_key.encrypt_integer(message);
    let decrypted: u64 = private_key.decrypt_integer(encrypted);

    println!("Message is {}", message);
    println!("Encrypted is {}", encrypted);
    println!("Decrypted is {}", decrypted);
}

fn generate_keypair(prime_numbers: Vec<u64>) -> (PublicKey, PrivateKey) {
    let prime1: u64 = 61; // &primes[rand::thread_rng().gen_range(0..=prime_numbers.len())];
    let prime2: u64 = 53;

    let primes_product: u64 = prime1 * prime2;

    let carmichaels_totient: i64 = lowest_common_multiple((prime1 as i64) - 1, (prime2 as i64) - 1);

    let exponent: u64 = 17; // Small for testing purposes

    let multiplicative_inverse: u64 =
        multiplicitive_inverse(exponent as i64, carmichaels_totient as i64);

    let public_key: PublicKey = PublicKey {
        primes_product,
        exponent,
    };

    let private_key: PrivateKey = PrivateKey {
        primes_product,
        multiplicative_inverse,
    };

    (public_key, private_key)
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
}*/

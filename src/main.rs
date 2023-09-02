use rand::Rng;

fn main() {
    let primes: Vec<u32> = find_primes_smaller_than(10000);

    let prime1: &u32 = &primes[rand::thread_rng().gen_range(0..=primes.len())];
    let prime2: &u32 = &primes[rand::thread_rng().gen_range(0..=primes.len())];

    let primes_multiplied: u32 = prime1 * prime2;

    let lcm: u32 = lowest_common_multiple(prime1 - 1, prime2 - 1);

    let exponent: u32 = 65537;
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

fn main() {
    let primes: Vec<u32> = find_primes_smaller_than(100);

    for prime in primes {
        println!("{}", prime)
    }
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

// Uses Sieve of Eratosthenes algorithm
pub fn find_primes_smaller_than(number: u64) -> Vec<u64> {
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
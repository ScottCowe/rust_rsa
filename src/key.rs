use crate::arithmetic::modular_exponent;

pub struct PublicKey {
    pub primes_product: u64,
    pub exponent: u64,
}

impl PublicKey {
    pub fn encrypt_integer(&self, to_encrypt: u64) -> u64 {
        let result: u64 = modular_exponent(to_encrypt, self.exponent, self.primes_product);
        result
    }
}

pub struct PrivateKey {
    pub primes_product: u64,
    pub multiplicative_inverse: u64,
}

impl PrivateKey {
    pub fn decrypt_integer(&self, to_decrypt: u64) -> u64 {
        let result: u64 =
            modular_exponent(to_decrypt, self.multiplicative_inverse, self.primes_product);
        result
    }
}

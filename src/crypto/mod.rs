//! Cryptographic primitives for Dataprune

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use rand::RngCore;
use x25519_dalek::{EphemeralSecret, PublicKey};

pub struct Cipher {
    cipher: Aes256Gcm,
}

pub struct KeyExchange {
    secret: EphemeralSecret,
    public: PublicKey,
}

impl Cipher {
    pub fn new(key: &[u8; 32]) -> Self {
        let cipher = Aes256Gcm::new_from_slice(key).expect("valid key length");
        Self { cipher }
    }
    
    pub fn encrypt(&self, plaintext: &[u8], nonce: &[u8; 12]) -> Vec<u8> {
        let nonce = Nonce::from_slice(nonce);
        self.cipher.encrypt(nonce, plaintext)
            .expect("encryption failure")
    }
    
    pub fn decrypt(&self, ciphertext: &[u8], nonce: &[u8; 12]) -> Vec<u8> {
        let nonce = Nonce::from_slice(nonce);
        self.cipher.decrypt(nonce, ciphertext)
            .expect("decryption failure")
    }
}

impl KeyExchange {
    pub fn new() -> Self {
        let secret = EphemeralSecret::random_from_rng(rand::thread_rng());
        let public = PublicKey::from(&secret);
        Self { secret, public }
    }
    
    pub fn public_key(&self) -> &[u8; 32] {
        self.public.as_bytes()
    }
    
    pub fn shared_secret(self, other_public: &PublicKey) -> [u8; 32] {
        *self.secret.diffie_hellman(other_public).as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encryption_roundtrip() {
        let key = [0u8; 32];
        let cipher = Cipher::new(&key);
        let nonce = [0u8; 12];
        let plaintext = b"hello world";
        
        let ciphertext = cipher.encrypt(plaintext, &nonce);
        let decrypted = cipher.decrypt(&ciphertext, &nonce);
        
        assert_eq!(plaintext.to_vec(), decrypted);
    }
}

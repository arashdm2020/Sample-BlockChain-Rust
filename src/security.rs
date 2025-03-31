use std::error::Error;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jwt::{encode, decode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use ed25519_dalek::{Keypair, PublicKey, SecretKey, Signature};
use rand::rngs::OsRng;
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use hmac::{Hmac, Mac};
use sha1::Sha1;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

pub struct Security {
    jwt_secret: Vec<u8>,
    encryption_key: Aes256Gcm,
}

impl Security {
    pub fn new(jwt_secret: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let encryption_key = Aes256Gcm::new(&jwt_secret[..32].into());
        Ok(Security {
            jwt_secret,
            encryption_key,
        })
    }

    // Password hashing using Argon2
    pub fn hash_password(&self, password: &str) -> Result<String, Box<dyn Error>> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(password_hash.to_string())
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, Box<dyn Error>> {
        let parsed_hash = PasswordHash::new(hash)?;
        Ok(Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }

    // JWT token generation and verification
    pub fn generate_token(&self, user_id: &str) -> Result<String, Box<dyn Error>> {
        let claims = JwtClaims {
            sub: user_id.to_string(),
            exp: chrono::Utc::now().timestamp() + 3600, // 1 hour expiration
            iat: chrono::Utc::now().timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(&self.jwt_secret),
        )?;

        Ok(token)
    }

    pub fn verify_token(&self, token: &str) -> Result<JwtClaims, Box<dyn Error>> {
        let validation = Validation::default();
        let token_data = decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(&self.jwt_secret),
            &validation,
        )?;

        Ok(token_data.claims)
    }

    // Digital signature generation and verification
    pub fn generate_keypair(&self) -> (PublicKey, SecretKey) {
        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(&mut csprng);
        (keypair.public, keypair.secret)
    }

    pub fn sign_message(&self, message: &[u8], secret_key: &SecretKey) -> Result<Signature, Box<dyn Error>> {
        let keypair = Keypair::from_bytes(&secret_key.to_bytes())?;
        Ok(keypair.sign(message))
    }

    pub fn verify_signature(
        &self,
        message: &[u8],
        signature: &Signature,
        public_key: &PublicKey,
    ) -> Result<bool, Box<dyn Error>> {
        Ok(public_key.verify(message, signature).is_ok())
    }

    // End-to-end encryption
    pub fn encrypt_message(&self, message: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let nonce = Nonce::from_slice(b"unique nonce"); // In production, use a unique nonce for each message
        let ciphertext = self.encryption_key.encrypt(nonce, message)?;
        Ok(ciphertext)
    }

    pub fn decrypt_message(&self, ciphertext: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        let nonce = Nonce::from_slice(b"unique nonce"); // Must match the nonce used for encryption
        let plaintext = self.encryption_key.decrypt(nonce, ciphertext)?;
        Ok(plaintext)
    }

    // HMAC for message integrity
    pub fn generate_hmac(&self, message: &[u8]) -> Result<Vec<u8>, Box<dyn Error>> {
        type HmacSha1 = Hmac<Sha1>;
        let mut mac = HmacSha1::new_from_slice(&self.jwt_secret)?;
        mac.update(message);
        Ok(mac.finalize().into_bytes().to_vec())
    }

    pub fn verify_hmac(&self, message: &[u8], hmac: &[u8]) -> Result<bool, Box<dyn Error>> {
        type HmacSha1 = Hmac<Sha1>;
        let mut mac = HmacSha1::new_from_slice(&self.jwt_secret)?;
        mac.update(message);
        Ok(mac.verify_slice(hmac).is_ok())
    }
}

// Two-factor authentication
pub struct TwoFactorAuth {
    security: Security,
}

impl TwoFactorAuth {
    pub fn new(security: Security) -> Self {
        TwoFactorAuth { security }
    }

    pub fn generate_totp(&self, secret: &[u8]) -> Result<String, Box<dyn Error>> {
        // TODO: Implement Time-based One-Time Password (TOTP) generation
        Ok("123456".to_string()) // Placeholder
    }

    pub fn verify_totp(&self, secret: &[u8], code: &str) -> Result<bool, Box<dyn Error>> {
        // TODO: Implement TOTP verification
        Ok(true) // Placeholder
    }
}

// Intrusion Detection System
pub struct IntrusionDetection {
    failed_attempts: std::sync::Mutex<std::collections::HashMap<String, u32>>,
    lockout_duration: chrono::Duration,
}

impl IntrusionDetection {
    pub fn new() -> Self {
        IntrusionDetection {
            failed_attempts: std::sync::Mutex::new(std::collections::HashMap::new()),
            lockout_duration: chrono::Duration::minutes(15),
        }
    }

    pub fn record_failed_attempt(&self, ip: &str) -> bool {
        let mut attempts = self.failed_attempts.lock().unwrap();
        let count = attempts.entry(ip.to_string()).or_insert(0);
        *count += 1;
        *count <= 5 // Allow up to 5 failed attempts
    }

    pub fn reset_failed_attempts(&self, ip: &str) {
        let mut attempts = self.failed_attempts.lock().unwrap();
        attempts.remove(ip);
    }
} 
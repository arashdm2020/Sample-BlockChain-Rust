use std::error::Error;
use serde::{Serialize, Deserialize};
use sysinfo::{System, SystemExt};
use sha2::{Sha256, Digest};
use ed25519_dalek::{Keypair, PublicKey, SecretKey};
use rand::rngs::OsRng;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub id: String,
    pub email: String,
    pub public_key: Vec<u8>,
    pub hardware_id: String,
    pub balance: f64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WalletCredentials {
    pub email: String,
    pub pin: String,  // 7-digit number
}

impl Wallet {
    pub fn new(email: String, pin: String) -> Result<Self, Box<dyn Error>> {
        // Generate hardware ID based on system information
        let sys = System::new_all();
        let hardware_id = generate_hardware_id(&sys);
        
        // Generate keypair
        let mut csprng = OsRng{};
        let keypair: Keypair = Keypair::generate(&mut csprng);
        
        // Create wallet address (16 characters)
        let address = generate_wallet_address(&keypair.public);
        
        Ok(Wallet {
            id: Uuid::new_v4().to_string(),
            email,
            public_key: keypair.public.to_bytes().to_vec(),
            hardware_id,
            balance: 0.0,
            created_at: chrono::Utc::now(),
        })
    }

    pub fn verify_hardware(&self) -> bool {
        let sys = System::new_all();
        let current_hardware_id = generate_hardware_id(&sys);
        self.hardware_id == current_hardware_id
    }
}

fn generate_hardware_id(sys: &System) -> String {
    let mut hasher = Sha256::new();
    
    // Collect system information
    if let Some(cpu) = sys.cpu_info().first() {
        hasher.update(cpu.brand().as_bytes());
    }
    if let Some(memory) = sys.memory_info() {
        hasher.update(memory.total.to_string().as_bytes());
    }
    
    // Generate a unique hardware ID
    let result = hasher.finalize();
    format!("{:x}", result)[..16].to_string()
}

fn generate_wallet_address(public_key: &PublicKey) -> String {
    let mut hasher = Sha256::new();
    hasher.update(public_key.to_bytes());
    let result = hasher.finalize();
    
    // Convert to base58 and take first 16 characters
    let address = bs58::encode(result).into_string();
    address[..16].to_string()
}

pub fn create_wallet(email: String, pin: String) -> Result<Wallet, Box<dyn Error>> {
    // Validate email format
    if !email.contains('@') {
        return Err("Invalid email format".into());
    }
    
    // Validate PIN format (7 digits)
    if pin.len() != 7 || !pin.chars().all(|c| c.is_digit(10)) {
        return Err("PIN must be 7 digits".into());
    }
    
    Wallet::new(email, pin)
}

pub fn access_wallet(email: String, pin: String) -> Result<Wallet, Box<dyn Error>> {
    // TODO: Implement wallet access from database
    // This will verify the credentials and hardware ID
    unimplemented!()
} 
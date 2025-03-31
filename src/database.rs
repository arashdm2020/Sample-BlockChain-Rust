use std::error::Error;
use mysql::*;
use mysql::prelude::*;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub username: String,
    pub password: String,
    pub database: String,
    pub host: String,
    pub port: u16,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            username: "root".to_string(),
            password: "".to_string(),
            database: "blockchain".to_string(),
            host: "localhost".to_string(),
            port: 3306,
        }
    }
}

pub struct Database {
    pool: Pool,
}

impl Database {
    pub fn new(config: DatabaseConfig) -> Result<Self, Box<dyn Error>> {
        let url = format!(
            "mysql://{}:{}@{}:{}/{}",
            config.username, config.password, config.host, config.port, config.database
        );
        
        let pool = Pool::new(url)?;
        Ok(Database { pool })
    }

    pub fn init_database(&self) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        
        // Create tables
        conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS wallets (
                id VARCHAR(36) PRIMARY KEY,
                email VARCHAR(255) UNIQUE NOT NULL,
                public_key BLOB NOT NULL,
                hardware_id VARCHAR(64) NOT NULL,
                balance DECIMAL(20,8) DEFAULT 0,
                created_at DATETIME NOT NULL
            )"
        )?;

        conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS blocks (
                hash VARCHAR(64) PRIMARY KEY,
                previous_hash VARCHAR(64) NOT NULL,
                timestamp DATETIME NOT NULL,
                poh_hash VARCHAR(64) NOT NULL,
                poh_count BIGINT NOT NULL
            )"
        )?;

        conn.query_drop(
            r"CREATE TABLE IF NOT EXISTS transactions (
                id VARCHAR(36) PRIMARY KEY,
                block_hash VARCHAR(64),
                from_address VARCHAR(16) NOT NULL,
                to_address VARCHAR(16) NOT NULL,
                amount DECIMAL(20,8) NOT NULL,
                timestamp DATETIME NOT NULL,
                signature BLOB NOT NULL,
                FOREIGN KEY (block_hash) REFERENCES blocks(hash)
            )"
        )?;

        Ok(())
    }

    pub fn save_wallet(&self, wallet: &crate::wallet::Wallet) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        
        conn.exec_drop(
            r"INSERT INTO wallets (id, email, public_key, hardware_id, balance, created_at)
              VALUES (?, ?, ?, ?, ?, ?)",
            (
                wallet.id,
                wallet.email,
                wallet.public_key.as_slice(),
                wallet.hardware_id,
                wallet.balance,
                wallet.created_at
            )
        )?;

        Ok(())
    }

    pub fn get_wallet(&self, email: &str) -> Result<Option<crate::wallet::Wallet>, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        
        let result = conn.query_map(
            r"SELECT id, email, public_key, hardware_id, balance, created_at
              FROM wallets WHERE email = ?",
            (email,),
            |(id, email, public_key, hardware_id, balance, created_at)| {
                crate::wallet::Wallet {
                    id,
                    email,
                    public_key: public_key.to_vec(),
                    hardware_id,
                    balance,
                    created_at,
                }
            }
        )?;

        Ok(result.into_iter().next())
    }

    pub fn save_block(&self, block: &crate::blockchain::Block) -> Result<(), Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        
        conn.exec_drop(
            r"INSERT INTO blocks (hash, previous_hash, timestamp, poh_hash, poh_count)
              VALUES (?, ?, ?, ?, ?)",
            (
                block.hash,
                block.previous_hash,
                block.timestamp,
                block.poh_hash,
                block.poh_count
            )
        )?;

        // Save transactions
        for transaction in &block.transactions {
            conn.exec_drop(
                r"INSERT INTO transactions (id, block_hash, from_address, to_address, amount, timestamp, signature)
                  VALUES (?, ?, ?, ?, ?, ?, ?)",
                (
                    transaction.id,
                    block.hash,
                    transaction.from,
                    transaction.to,
                    transaction.amount,
                    transaction.timestamp,
                    transaction.signature.as_slice()
                )
            )?;
        }

        Ok(())
    }

    pub fn get_latest_block(&self) -> Result<Option<crate::blockchain::Block>, Box<dyn Error>> {
        let mut conn = self.pool.get_conn()?;
        
        let result = conn.query_map(
            r"SELECT hash, previous_hash, timestamp, poh_hash, poh_count
              FROM blocks ORDER BY timestamp DESC LIMIT 1",
            (),
            |(hash, previous_hash, timestamp, poh_hash, poh_count)| {
                crate::blockchain::Block {
                    hash,
                    previous_hash,
                    timestamp,
                    transactions: vec![], // Transactions will be loaded separately
                    poh_hash,
                    poh_count,
                }
            }
        )?;

        Ok(result.into_iter().next())
    }
} 
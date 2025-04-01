# Rust Blockchain Network

A high-performance blockchain network implementation in Rust with advanced features including Proof of History consensus, decentralized market, and governance system.

## Features

- **High-Speed Transactions**: Optimized for fast transaction processing
- **Proof of History Consensus**: Efficient and secure consensus mechanism
- **Decentralized Market**: Built-in token trading platform
- **Governance System**: On-chain voting and proposal management
- **Advanced Security**: Hardware-based identification and encryption
- **Sharding Support**: Scalable architecture with sharding capabilities

## Technical Stack

- **Language**: Rust
- **Database**: MySQL
- **API**: REST + WebSocket
- **Security**: JWT, AES-GCM, Argon2
- **Consensus**: Proof of History
- **Networking**: Custom P2P protocol

## Prerequisites

- Rust 1.70 or later
- MySQL 8.0 or later
- Visual Studio Build Tools (for Windows)
- Git

## Installation

1. Clone the repository:
```bash
git clone https://github.com/arashdm2020/Sample-BlockChain-Rust.git
cd Sample-BlockChain-Rust
```

2. Install dependencies:
```bash
cargo build --release
```

3. Configure environment variables:
```bash
cp .env.example .env
# Edit .env with your configuration
```

4. Set up the database:
```bash
mysql -u root -p < scripts/init_db.sql
```

## Configuration

The project uses environment variables for configuration. See `.env.example` for all available options.

## Usage

1. Start the node:
```bash
cargo run --release
```

2. Access the API:
- REST API: http://localhost:8080
- WebSocket: ws://localhost:8081

## Project Structure

```
Sample-BlockChain-Rust/
├── src/
│   ├── api/          # API endpoints
│   ├── blockchain/   # Core blockchain implementation
│   ├── consensus/    # Proof of History consensus
│   ├── database/     # Database operations
│   ├── market/       # Decentralized market
│   ├── network/      # P2P networking
│   ├── security/     # Security features
│   └── governance/   # Governance system
├── tests/            # Test suite
├── scripts/          # Utility scripts
└── docs/            # Documentation
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Author

- **Arash Dolati Mehr**
  - Email: arash.dolati.m@live.com
  - LinkedIn: [Arash Dolati Mehr](https://www.linkedin.com/in/arashdolatimehr/)

## Acknowledgments

- Thanks to all contributors who have helped shape this project
- Special thanks to the Rust community for their excellent tools and libraries 

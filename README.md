# Chinese Blockchain Network

A high-performance blockchain network implemented in Rust with Proof of History consensus, featuring a decentralized market, governance system, and advanced security features.

## Features

- **High-Speed Transactions**
  - Proof of History (PoH) consensus algorithm
  - Parallel transaction processing pipeline
  - System sharding for network load sharing

- **Wallet System**
  - 16-character addresses with letters and symbols
  - Hardware-based authentication
  - Email and PIN-based wallet recovery

- **Market System**
  - Decentralized exchange
  - Smart contracts
  - Atomic swaps
  - Token price management

- **Governance**
  - Community voting system
  - Protocol improvement proposals
  - Community development budget
  - Voting power calculation

- **Security**
  - End-to-end encryption
  - Two-factor authentication
  - Intrusion detection
  - Digital signatures

- **API System**
  - REST API endpoints
  - WebSocket for real-time notifications
  - Python and JavaScript client libraries

## Prerequisites

- Rust 1.70 or later
- MySQL 8.0 or later
- Node.js 16 or later (for JavaScript client)
- Python 3.8 or later (for Python client)

## Installation

1. Clone the repository:
```bash
git clone https://github.com/yourusername/chinese-blockchain.git
cd chinese-blockchain
```

2. Set up the database:
```sql
CREATE DATABASE blockchain;
CREATE USER 'blockchain'@'localhost' IDENTIFIED BY 'your_password';
GRANT ALL PRIVILEGES ON blockchain.* TO 'blockchain'@'localhost';
FLUSH PRIVILEGES;
```

3. Configure the environment:
```bash
cp .env.example .env
# Edit .env with your database credentials and other settings
```

4. Build the project:
```bash
cargo build --release
```

## Usage

1. Start the blockchain node:
```bash
cargo run --release
```

2. Access the API:
- REST API: http://localhost:8080/api
- WebSocket: ws://localhost:8080/ws

3. Use the client libraries:

Python:
```python
from chinese_blockchain import BlockchainClient

client = BlockchainClient("http://localhost:8080")
wallet = client.create_wallet("user@example.com", "1234567")
balance = client.get_balance()
```

JavaScript:
```javascript
import { BlockchainClient } from 'chinese-blockchain';

const client = new BlockchainClient("http://localhost:8080");
const wallet = await client.createWallet("user@example.com", "1234567");
const balance = await client.getBalance();
```

## API Documentation

### REST API Endpoints

#### Wallet
- `POST /api/wallet` - Create new wallet
- `GET /api/wallet/balance` - Get wallet balance
- `POST /api/wallet/transfer` - Transfer tokens

#### Market
- `GET /api/market/price/:symbol` - Get token price
- `POST /api/market/order` - Place order
- `GET /api/market/orders` - Get order book

#### Governance
- `POST /api/governance/proposal` - Create proposal
- `POST /api/governance/vote` - Cast vote
- `GET /api/governance/proposals` - List proposals

### WebSocket Events

- `wallet.created` - New wallet created
- `transaction.confirmed` - Transaction confirmed
- `price.updated` - Token price updated
- `proposal.created` - New proposal created
- `vote.cast` - Vote cast

## Security Considerations

1. Always use HTTPS in production
2. Keep your private keys secure
3. Enable two-factor authentication
4. Regularly update dependencies
5. Monitor for suspicious activity

## Contributing

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Create a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Solana for the Proof of History concept
- Ethereum for smart contract inspiration
- Bitcoin for blockchain fundamentals 
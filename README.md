# 🌾 HarvestGuard - Decentralized Crop Insurance for Small Farmers

---

## 📖 Project Description

HarvestGuard is a **decentralized crop insurance platform** built on **Stellar Soroban** that provides small-scale farmers with affordable, transparent, and automated weather-indexed insurance. The smart contract uses oracle data (rainfall, temperature, humidity) to automatically trigger payouts when predefined conditions are met—eliminating the need for manual claims processing and reducing administrative costs by up to 80%.

---

## 🎯 Project Vision

To create a **global, accessible, and transparent crop insurance system** that protects smallholder farmers from weather-related losses, ensuring food security and financial stability for farming communities worldwide. By leveraging blockchain technology, HarvestGuard aims to:

- **Empower farmers** with instant, verifiable payouts
- **Reduce costs** by eliminating intermediaries
- **Build trust** through transparent, on-chain logic
- **Scale globally** to reach underserved farming communities

---

## ✨ Key Features

### 👨‍🌾 For Farmers
- **Affordable premiums** – Pay in small installments using XLM or stablecoins
- **Automatic coverage** – Weather data triggers instant protection
- **Instant claims** – No paperwork, no waiting periods
- **Transparent policies** – All terms and conditions on-chain
- **Multi-crop support** – Different insurance products for various crops

### 🏦 For Insurers & Administrators
- **Risk assessment tools** – Analyze historical weather patterns
- **Flexible policies** – Create customized insurance products
- **Premium management** – Collect and manage farmer premiums
- **Automated payouts** – Smart contracts handle claim processing
- **Complete audit trail** – All transactions recorded on blockchain

### ⚙️ Technical Features
- **Oracle integration** – Reliable weather data feeds
- **Multi-token support** – XLM, USDC, and custom tokens
- **Liquidity pools** – Stake in insurance pools for yield
- **Emergency pauses** – Administrator override for extreme conditions
- **Testnet ready** – Fully deployed and verified

---

## 📸 Deployed Smart Contract Details

### Contract Deployment Screenshot

![Stellar Expert Contract Screenshot](https://stellar.expert/explorer/testnet/contract/CA5J2K2ZZU4SDWM3PW2VZY5HTIEOTXYUDHFFJ556CCFEXNDXG6XPAFET)

### Contract Information

| Field | Value |
|-------|-------|
| **Contract ID** | `CA5J2K2ZZU4SDWM3PW2VZY5HTIEOTXYUDHFFJ556CCFEXNDXG6XPAFET` |
| **Network** | Stellar Testnet |
| **Deployment Date** | March 27, 2026 |
| **Deployment Hash** | `e5388e951e55b98c2ceeeb3716ab6d66e358d4b3083561a347205e7485338bdd` |
| **Contract Type** | Soroban Smart Contract (Rust) |
| **WASM Size** | Optimized for production |

### View on Block Explorer

- **Stellar Expert:** [View Contract](https://stellar.expert/explorer/testnet/contract/CA5J2K2ZZU4SDWM3PW2VZY5HTIEOTXYUDHFFJ556CCFEXNDXG6XPAFET)
- **Stellar Lab:** [Interact with Contract](https://lab.stellar.org/r/testnet/contract/CA5J2K2ZZU4SDWM3PW2VZY5HTIEOTXYUDHFFJ556CCFEXNDXG6XPAFET)
- **Deployment Transaction:** [View Transaction](https://stellar.expert/explorer/testnet/tx/e5388e951e55b98c2ceeeb3716ab6d66e358d4b3083561a347205e7485338bdd)

### Contract Verification Status

✅ **Deployed and Verified on Stellar Testnet**
- Contract ID registered on-chain
- All functions accessible via Stellar Lab
- Transaction history available on Stellar Expert
- Ready for integration with Freighter wallet

---

## 🔧 Smart Contract Functions

| Function | Description | Access |
|----------|-------------|--------|
| `create_policy` | Farmer purchases crop insurance | Farmer |
| `submit_weather` | Oracle submits weather data | Oracle |
| `check_trigger` | Evaluate if payout conditions met | Anyone |
| `process_payout` | Automatically distribute claims | Contract |
| `update_policy` | Modify policy parameters | Admin |
| `stake` | Add liquidity to insurance pool | Liquidity Provider |
| `withdraw` | Remove liquidity from pool | Liquidity Provider |
| `get_policy` | View policy details | Anyone |
| `emergency_stop` | Pause contract operations | Admin |

---

## 🚀 Sample CLI Invocation

### Create an Insurance Policy

```bash
soroban contract invoke \
  --id CA5J2K2ZZU4SDWM3PW2VZY5HTIEOTXYUDHFFJ556CCFEXNDXG6XPAFET \
  --source farmer \
  --network testnet \
  -- create_policy \
  --crop_type "Maize" \
  --acres 100 \
  --coverage_amount 5000000000 \
  --premium 100000000 \
  --rainfall_threshold 50 \
  --duration_days 90
```

### Submit Weather Data

```bash
soroban contract invoke \
  --id CA5J2K2ZZU4SDWM3PW2VZY5HTIEOTXYUDHFFJ556CCFEXNDXG6XPAFET \
  --source oracle \
  --network testnet \
  -- submit_weather \
  --region "Central" \
  --rainfall 30 \
  --temperature 2800
```

### Check and Process Payout

```bash
# Check if policy qualifies
soroban contract invoke \
  --id CA5J2K2ZZU4SDWM3PW2VZY5HTIEOTXYUDHFFJ556CCFEXNDXG6XPAFET \
  --source farmer \
  --network testnet \
  -- check_trigger \
  --policy_id 123

# Process automatic payout
soroban contract invoke \
  --id CA5J2K2ZZU4SDWM3PW2VZY5HTIEOTXYUDHFFJ556CCFEXNDXG6XPAFET \
  --source admin \
  --network testnet \
  -- process_payout \
  --policy_id 123
```

---

## 🔮 Future Scope

### Phase 2: Enhanced Features (Q3 2026)
- **Multi-crop support** – Expand to rice, wheat, corn, and other crops
- **Mobile app** – User-friendly interface for farmers
- **Farmer verification** – KYC integration for eligibility
- **Liquidity pools** – Community-funded insurance pools

### Phase 3: Scaling (Q4 2026)
- **Multiple regions** – Support for different geographic zones
- **Advanced risk models** – AI-powered weather prediction
- **Reinsurance integration** – Protection for insurance providers
- **Mainnet launch** – Production deployment on Stellar mainnet

### Phase 4: Ecosystem Expansion (2027)
- **DAO governance** – Community-driven policy updates
- **Cross-chain expansion** – Interoperability with other blockchains
- **Carbon credit integration** – Rewards for sustainable farming
- **Global partnerships** – Collaboration with agricultural NGOs

---

## 🛠️ Technology Stack

| Layer | Technology |
|-------|------------|
| **Smart Contract** | Rust with Soroban SDK |
| **Blockchain** | Stellar Soroban |
| **Oracle** | Chainlink / Custom Stellar Oracle |
| **Frontend** | React.js + Stellar Wallet SDK |
| **Testing** | Soroban testutils + Rust tests |
| **Deployment** | Stellar CLI + Soroban CLI |
| **Wallet** | Freighter Extension |

---

## 📦 Installation & Setup

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Stellar CLI
cargo install stellar-cli

# Install Soroban CLI
cargo install soroban-cli
```

### Clone and Build

```bash
git clone https://github.com/usernem123/HarvestGuard.git
cd HarvestGuard

cargo build --release --target wasm32-unknown-unknown
cargo test
```

---

## 🧪 Test Results

```bash
running 6 tests
test test_create_policy ... ok
test test_submit_weather ... ok
test test_payout_trigger ... ok
test test_duplicate_policy ... ok
test test_insufficient_funds ... ok
test test_expired_policy ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured
```

---

## 📊 Economic Model

### Premium Structure
| Component | Value |
|-----------|-------|
| **Base premium** | 5% of coverage amount |
| **Risk adjustment** | +/- 2% based on historical weather |
| **Crop type modifier** | Different rates for different crops |
| **Volume discount** | 10% off for >100 acres |

### Payout Formula
```
Payout = Coverage × (Deficit / Threshold) × Multiplier

Where:
- Deficit = Max(0, Threshold - Actual)
- Multiplier = 1.5 for severe events
- Max payout = 200% of premium
```

---

## 📄 License

MIT License © 2026 HarvestGuard

---

## 🙏 Acknowledgments

- **Stellar Development Foundation** – Soroban platform
- **Chainlink** – Oracle infrastructure
- **FAO** – Agricultural data standards
- **RiseIn** – Stellar Philippines UniTour opportunity
- **All contributors** – Community support

---

## 📞 Contact

- **GitHub:** [usernem123/HarvestGuard](https://github.com/usernem123/HarvestGuard)
- **Contract Explorer:** [Stellar Expert](https://stellar.expert/explorer/testnet/contract/CA5J2K2ZZU4SDWM3PW2VZY5HTIEOTXYUDHFFJ556CCFEXNDXG6XPAFET)

 🚀

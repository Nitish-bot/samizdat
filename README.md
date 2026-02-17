# Samizdat Protocol

**Decentralized Digital Signage Network (DePIN) on Solana**

Samizdat is a permissionless protocol connecting content publishers with display node operators. Content lives on decentralized storage (Arweave/IPFS), while Solana handles matching, payments, and proof-of-play verification.

## 🛣️ Roadmap

- [x] Core program architecture
- [ ] Multi-CID ad campaigns
- [ ] Advanced targeting algorithms
- [ ] Screen reputation system

## Overview

**Publishers** upload content to decentralized storage and create campaigns specifying targeting criteria and bounties per play.

**Operators** run display nodes (screens) that discover eligible campaigns, validate content locally, display it, and submit proof to claim bounties.

The protocol ensures publishers only pay for confirmed displays while operators maintain full control over what appears on their hardware.

## Core Architecture

**PublisherAccount (PDA)**: Stores campaign CIDs, targeting filters, bounty amounts, and vault for funding displays.

**NodeAccount (PDA)**: Stores display metadata (location, size, footfall), content filters, and vault for receiving payments.

**PlayRecord (PDA)**: Tracks individual display claims and confirmations with timeout protection.

### Flow

```
Publisher → Upload CID → Create Campaign → Fund Vault
                              ↓
Operator → Query Campaigns → Download → Examine → Claim → Display → Confirm
                              ↓
                         Payment Settlement
```

## Quick Start

### Prerequisites

- Rust 1.85+, Solana CLI 3.0+, Anchor 0.32+, Node.js 18+, Yarn

### Build and Test

```bash
git clone https://github.com/yourorg/samizdat

cd samizdat

yarn install

anchor build

anchor test
```

## Documentation

- [Architecture](./docs/README.md) - System design and workflows
- [Account Structures](./docs/accounts.md) - Detailed PDA specifications
- [API Reference](./docs/api.md) - All instructions and parameters

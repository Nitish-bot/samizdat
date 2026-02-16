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
Operator → Query Campaigns → Claim → Download → Display → Confirm
                              ↓
                         Payment Settlement
```

### Flow

```
Publisher → Upload CID → Create Campaign → Fund Vault
                              ↓
Operator → Query Campaigns → Claim → Download → Display → Confirm
                              ↓
                         Payment Settlement
```

## Quick Start

### Prerequisites

- Rust 1.75+, Solana CLI 1.18+, Anchor 0.32+, Node.js 18+, Yarn

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
- [Deployment Guide](./docs/deployment.md) - Setup instructions

## Usage

### Publisher: Create Campaign

```typescript
const cid = await uploadToArweave(imageBuffer);

await program.methods
  .createPublisherAccount({
    cids: [cid],
    bountyPerPlay: new BN(1_000_000), // 0.001 SOL
    totalPlays: 1000,
    targetFilters: { minFootfall: 100 }
  })
  .accounts({ publisherAccount: pda, authority: publisher.publicKey })
  .rpc();
```

### Operator: Register Node

```typescript
await program.methods
  .registerNode({
    location: { lat: 40.7128, lon: -74.0060 },
    screenSize: "large",
    resolution: { width: 1920, height: 1080 },
    estimatedFootfall: 5000,
    contentFilters: ["no-adult", "family-friendly"]
  })
  .accounts({ nodeAccount: pda, authority: operator.publicKey })
  .rpc();
```

### Operator: Claim and Display

```typescript
// Query eligible campaigns
const campaigns = await program.account.publisherAccount.all();

// Claim campaign
await program.methods.claimCampaign()
  .accounts({ publisherAccount, nodeAccount, playRecord })
  .rpc();

// Download and validate
const content = await fetchFromArweave(campaign.cids[0]);
validateContent(content);

// Display and confirm
await program.methods.confirmPlay()
  .accounts({ playRecord, publisherAccount, nodeAccount })
  .rpc();
```

## Security

- Authority-gated mutations
- Timeout protection for unclaimed plays (5 minutes)
- Rent-exempt PDAs
- Local content validation by operators
- Filter-based targeting

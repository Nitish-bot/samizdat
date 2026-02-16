# Deployment Guide

## Prerequisites

- Rust 1.89
- Solana CLI 3
- Anchor CLI 0.32
- Node.js 18+ and Yarn

## Local Development

### 1. Clone and Install

```bash
git clone https://github.com/bakayu/samizdat
cd samizdat
yarn install
```

### 2. Build Program

```bash
anchor build
```

Program ID will be in `target/deploy/samizdat-keypair.json`.

### 3. Update Program ID

```bash
# Sync program ids
anchor keys sync
```

### 4. Run Tests

```bash
anchor test
```

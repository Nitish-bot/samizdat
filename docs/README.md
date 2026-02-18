# Samizdat Architecture

## Overview

Samizdat creates a two-sided marketplace for content distribution on physical displays. Publishers create campaigns with targeting criteria, operators run display nodes that autonomously discover and show eligible content.

## Actors

**Publisher**: Entity that creates campaigns, uploads content to decentralized storage, and funds displays.

**Operator**: Entity that runs display nodes (screens), discovers campaigns, validates content, and earns bounties.

## Core Concepts

### Content Storage
Content (images) lives off-chain on Arweave/IPFS, referenced by CID. Publishers upload content independently, then reference CIDs on-chain.

### Matching
Operators query on-chain state to discover campaigns. Filtering happens both on-chain (via PDAs) and locally (operator validates targeting criteria and content safety).

### Payment
Publishers pre-fund campaigns. When operators confirm display, bounty transfers from publisher vault to operator vault. If operator doesn't confirm within timeout, play count restores.

### Timeout Protection
After claiming a campaign, operators have 5 minutes to confirm display. If timeout expires without confirmation, anyone can call \`timeout_play\` to restore the play count.

## Account Structure

See [Account Reference](./accounts.md) for detailed specifications.

- **PublisherAccount**: Campaign state (CIDs, targeting, bounty, vault)
- **NodeAccount**: Display node state (location, specs, filters, vault)
- **PlayRecord**: Individual display tracking (claim, confirmation, payment)

## State Machine

### PublisherAccount States

\`\`\`
[Created] → Active → Paused ↔ Active → Depleted → Closed
\`\`\`

- **Active**: Accepting claims from operators
- **Paused**: Temporarily stopped by publisher
- **Depleted**: No plays or funds remaining
- **Closed**: Permanently closed, funds returned

### PlayRecord States

\`\`\`
[Created] → Claimed → Confirmed → Paid
                   ↘ TimedOut
\`\`\`

- **Claimed**: Operator claimed but hasn't confirmed
- **Confirmed**: Operator confirmed display
- **Paid**: Bounty transferred to operator
- **TimedOut**: Claim expired without confirmation

## Workflows

### Campaign Creation

\`\`\`mermaid
sequenceDiagram
    participant P as Publisher
    participant S as Storage
    participant Sol as Solana
    participant PDA as PublisherAccount

    P->>S: Upload image
    S-->>P: Return CID
    P->>Sol: create_publisher_account(CID, filters, bounty)
    Sol->>PDA: Initialize account
    P->>Sol: fund_publisher_account(amount)
    Sol->>PDA: Transfer to vault
\`\`\`

### Node Registration

\`\`\`mermaid
sequenceDiagram
    participant O as Operator
    participant Sol as Solana
    participant PDA as NodeAccount

    O->>Sol: register_node(location, specs, filters)
    Sol->>PDA: Initialize account
    Note over PDA: Node ready to claim campaigns
\`\`\`

### Display Cycle

\`\`\`mermaid
sequenceDiagram
    participant O as Operator
    participant Sol as Solana
    participant Pub as PublisherAccount
    participant Node as NodeAccount
    participant Play as PlayRecord
    participant S as Storage

    loop Every 30-60s
        O->>Sol: Query active campaigns
        Sol-->>O: Return matching campaigns
        O->>O: Filter by targeting & safety
        O->>Sol: claim_campaign()
        Sol->>Play: Create record (status: Claimed)
        Sol->>Pub: Decrement plays_remaining
        O->>S: Fetch content via CID
        S-->>O: Return image
        O->>O: Validate locally
        O->>O: Display on hardware
        O->>Sol: confirm_play()
        Sol->>Pub: Transfer bounty to NodeAccount
        Sol->>Play: Update status to Paid
        Sol->>Node: Increment earnings
    end
\`\`\`

### Timeout Recovery

\`\`\`mermaid
sequenceDiagram
    participant A as Anyone
    participant Sol as Solana
    participant Play as PlayRecord
    participant Pub as PublisherAccount

    Note over Play: 5 minutes passed since claim
    A->>Sol: timeout_play(play_record)
    Sol->>Sol: Verify timeout expired
    Sol->>Play: Update status to TimedOut
    Sol->>Pub: Restore plays_remaining
\`\`\`

## Matching Algorithm

Operators discover eligible campaigns using these filters:

### On-Chain Filters
- Campaign status == Active
- \`plays_remaining > 0\`
- Vault balance >= bounty_per_play

### Operator-Side Filters
- **Geographic**: Node location within campaign geo bounds (if specified)
- **Footfall**: Node footfall within min/max range (if specified)
- **Screen Size**: Node size matches campaign requirements (if specified)
- **Establishment**: Node type matches campaign targets (if specified)
- **Landmarks**: Node has required nearby landmarks (if specified)
- **Content Safety**: Campaign tags don't violate node content filters

Example filter check (operator-side):
\`\`\`rust
// Check geographic bounds
if let Some(bounds) = campaign.target_filters.geo_bounds {
    if node.location.lat < bounds.min_lat || node.location.lat > bounds.max_lat {
        return false; // Skip this campaign
    }
}

// Check content filters
for tag in &campaign.tags {
    for filter in &node.content_filters {
        if tag_violates_filter(tag, filter) {
            return false; // Skip this campaign
        }
    }
}
\`\`\`

## Security Model

### Authority Checks
All mutable operations require signature from account authority:
- Only publisher can modify PublisherAccount
- Only operator can modify NodeAccount and confirm plays

### Economic Security
- Publishers pre-fund campaigns (no credit)
- Rent-exempt PDAs prevent closure
- Timeout mechanism prevents griefing (operators can't indefinitely lock plays)

### Content Safety
- Operators validate content locally before display
- Operators maintain custom content filters
- Publishers can't change CIDs after creation (only add new ones)

### Double-Claim Prevention
Cannot create new PlayRecord if unclaimed one exists for same campaign-node pair.

## Instructions

See [API Reference](./api.md) for detailed instruction specifications.

**Publisher Instructions:**
- \`create_publisher_account\` - Create new campaign
- \`fund_publisher_account\` - Add funds to vault
- \`update_publisher_metadata\` - Update targeting/status
- \`add_cids_to_campaign\` - Add more content
- \`close_publisher_account\` - Close campaign and withdraw

**Operator Instructions:**
- \`register_node\` - Register display node
- \`update_node_metadata\` - Update node info
- \`claim_campaign\` - Claim campaign for display
- \`confirm_play\` - Confirm content was displayed

**Public Instructions:**
- \`timeout_play\` - Recover from expired claim (callable by anyone)

## Performance Considerations

### Polling Strategy
Operators should:
- Poll every 30-60 seconds (not more frequently)
- Use \`getProgramAccounts\` with memcmp filters for status
- Cache locally to reduce RPC load
- Use Geyser/indexer for production

### Account Sizes
- PublisherAccount: ~1200 bytes
- NodeAccount: ~800 bytes
- PlayRecord: ~200 bytes

### Scaling
For high-throughput deployments:
- Use Geyser plugin to index accounts
- Build Web2 API for fast campaign discovery
- WebSocket subscriptions for real-time updates
- Redis cache for hot campaigns

## Future Enhancements

- Video content support (duration-based pricing)
- Real-time bidding for high-value displays
- Reputation scoring for operators
- Analytics and reporting dashboard
- Cross-chain content delivery
- NFT-based content campaigns

---

**See Also:**
- [API Reference](./api.md)

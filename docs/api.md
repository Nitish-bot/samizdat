# Samizdat Protocol Reference

Complete reference for account structures, instructions, and error codes.

## Account Structures

### PublisherAccount (PDA)

**Seeds**: `["publisher", authority.key()]`

Represents a publisher entity. Required to create campaigns.

```rust
#[account]
pub struct PublisherAccount {
    /// Publisher authority
    pub authority: Pubkey,
    
    /// Total campaigns created
    pub total_campaigns: u64,
    
    /// Total amount spent across all campaigns (lamports)
    pub total_spent: u64,
    
    /// Registration timestamp
    pub registered_at: i64,
    
    /// Publisher status
    pub status: PublisherStatus,
    
    /// PDA bump seed
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum PublisherStatus {
    Active,
    Suspended,
}
```

**Size**: ~100 bytes

---

### CampaignAccount (PDA)

**Seeds**: `["campaign", publisher_account.key(), campaign_id.to_le_bytes()]`

Stores individual campaign details including content CIDs, targeting filters, bounty rates, and vault.

```rust
#[account]
pub struct CampaignAccount {
    /// Parent publisher account
    pub publisher_account: Pubkey,
    
    /// Unique campaign identifier (scoped to publisher)
    pub campaign_id: u64,
    
    /// Content IDs from decentralized storage (max 10)
    pub cids: Vec<String>,
    
    /// Payment per confirmed display (lamports)
    pub bounty_per_play: u64,
    
    /// Remaining displays available
    pub plays_remaining: u64,
    
    /// Total displays completed
    pub plays_completed: u64,
    
    /// Content tags for matching
    pub tags: Vec<String>,
    
    /// Targeting criteria
    pub target_filters: TargetFilters,
    
    /// Campaign status
    pub status: CampaignStatus,
    
    /// Creation timestamp
    pub created_at: i64,
    
    /// PDA bump seed
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct TargetFilters {
    pub min_footfall: Option<u32>,
    pub max_footfall: Option<u32>,
    pub screen_sizes: Vec<ScreenSize>,
    pub geo_bounds: Option<GeoBounds>,
    pub establishment_types: Vec<String>,
    pub required_landmarks: Vec<String>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GeoBounds {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum CampaignStatus {
    Active,
    Paused,
    Depleted,
    Closed,
}
```

**Size**: ~1200 bytes (variable based on vector lengths)

---

### NodeAccount (PDA)

**Seeds**: `["node_account", authority.key(), node_id.to_le_bytes()]`

Stores display node specifications, location, content filters, and earnings vault.

```rust
#[account]
pub struct NodeAccount {
    /// Node operator and authority
    pub authority: Pubkey,
    
    /// Unique node identifier
    pub node_id: u64,
    
    /// Physical location
    pub location: GeoLocation,
    
    /// Display specifications
    pub screen_size: ScreenSize,
    pub resolution: Resolution,
    
    /// Nearby points of interest
    pub landmarks: Vec<String>,
    
    /// Content safety filters
    pub content_filters: Vec<String>,
    
    /// Estimated daily foot traffic
    pub estimated_footfall: u32,
    
    /// Location type
    pub establishment_type: String,
    
    /// Total displays completed
    pub total_plays: u64,
    
    /// Total earnings (lamports)
    pub total_earnings: u64,
    
    /// Registration timestamp
    pub registered_at: i64,
    
    /// Node status
    pub status: NodeStatus,
    
    /// PDA bump seed
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct GeoLocation {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum ScreenSize {
    Small,   // < 32"
    Medium,  // 32-55"
    Large,   // 55-80"
    XLarge,  // > 80" (billboards)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum NodeStatus {
    Active,
    Offline,
    Suspended,
}
```

**Size**: ~800 bytes

---

#### PlayRecord (PDA)

**Seeds**: `["play_record", campaign_account.key(), node_account.key(), claim_timestamp.to_le_bytes()]`

Tracks individual display claims, confirmations, and payment settlements.

```rust
#[account]
pub struct PlayRecord {
    /// Associated campaign
    pub campaign_account: Pubkey,
    
    /// Associated display node
    pub node_account: Pubkey,
    
    /// Claim timestamp
    pub claimed_at: i64,
    
    /// Confirmation timestamp (0 if not confirmed)
    pub confirmed_at: i64,
    
    /// Index of CID displayed
    pub cid_index: u8,
    
    /// Amount paid (lamports)
    pub payment_amount: u64,
    
    /// Record status
    pub status: PlayStatus,
    
    /// PDA bump seed
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq)]
pub enum PlayStatus {
    Claimed,    // Node claimed, not yet confirmed
    Confirmed,  // Node confirmed display
    TimedOut,   // Claim expired
    Paid,       // Payment settled
}
```

**Size**: ~200 bytes

---

## Account Relationships

```
PublisherAccount (1) ──── (N) CampaignAccount (1) ────┬──── (N) PlayRecord ──── (N) NodeAccount (1)
                                                       │
                                                       └──── Vault (SOL)
                         
NodeAccount (1) ─────────────── Vault (SOL)
```

- One PublisherAccount can have many CampaignAccounts
- One CampaignAccount can have many PlayRecords
- One NodeAccount can have many PlayRecords
- Each PlayRecord links exactly one CampaignAccount to one NodeAccount
- CampaignAccount maintains a vault for payment settlement
- NodeAccount maintains a vault for receiving payments

---

## PDA Derivation Examples

```rust
// PublisherAccount PDA
let (publisher_pda, bump) = Pubkey::find_program_address(
    &[
        b"publisher",
        authority.key().as_ref(),
    ],
    program_id,
);

// CampaignAccount PDA
let (campaign_pda, bump) = Pubkey::find_program_address(
    &[
        b"campaign",
        publisher_account.key().as_ref(),
        &campaign_id.to_le_bytes(),
    ],
    program_id,
);

// NodeAccount PDA
let (node_pda, bump) = Pubkey::find_program_address(
    &[
        b"node_account",
        authority.key().as_ref(),
        &node_id.to_le_bytes(),
    ],
    program_id,
);

// PlayRecord PDA
let (play_pda, bump) = Pubkey::find_program_address(
    &[
        b"play_record",
        campaign_account.key().as_ref(),
        node_account.key().as_ref(),
        &claim_timestamp.to_le_bytes(),
    ],
    program_id,
);
```

---

## Instructions

### Publisher Instructions

#### register_publisher

Creates a PublisherAccount for a new publisher entity.

**Accounts:**
- `publisher_account` (init, pda) - PublisherAccount to create
- `authority` (signer, mut) - Publisher authority
- `system_program` - System program

**Args:** None

---

#### create_campaign

Creates a new campaign with content CIDs and targeting criteria.

**Accounts:**
- `campaign_account` (init, pda) - CampaignAccount to create
- `publisher_account` (mut) - Parent PublisherAccount
- `authority` (signer, mut) - Publisher authority
- `system_program` - System program

**Args:**
```rust
pub struct CreateCampaignArgs {
    pub campaign_id: u64,
    pub cids: Vec<String>,              // 1-10 CIDs
    pub bounty_per_play: u64,           // lamports
    pub total_plays: u64,
    pub tags: Vec<String>,
    pub target_filters: TargetFilters,
}
```

**Validation:**
- Publisher must be Active
- `cids.len()` between 1 and 10
- `bounty_per_play > 0`
- `total_plays > 0`
- `authority` matches `publisher_account.authority`

**Side Effects:**
- Increments `publisher_account.total_campaigns`

---

#### fund_campaign

Adds SOL to campaign vault to fund displays.

**Accounts:**
- `campaign_account` (mut) - CampaignAccount to fund
- `publisher_account` - Parent PublisherAccount
- `authority` (signer, mut) - Publisher authority
- `system_program` - System program

**Args:**
```rust
pub struct FundCampaignArgs {
    pub amount: u64,  // lamports to transfer
}
```

**Validation:**
- `authority` matches `publisher_account.authority`
- Campaign status is Active or Paused
- `amount > 0`

---

#### update_campaign

Updates campaign targeting, tags, or status.

**Accounts:**
- `campaign_account` (mut) - CampaignAccount to update
- `publisher_account` - Parent PublisherAccount
- `authority` (signer) - Publisher authority

**Args:**
```rust
pub struct UpdateCampaignArgs {
    pub tags: Option<Vec<String>>,
    pub target_filters: Option<TargetFilters>,
    pub status: Option<CampaignStatus>,
}
```

**Validation:**
- `authority` matches `publisher_account.authority`

---

#### add_cids_to_campaign

Adds additional content CIDs to existing campaign.

**Accounts:**
- `campaign_account` (mut) - CampaignAccount to update
- `publisher_account` - Parent PublisherAccount
- `authority` (signer) - Publisher authority

**Args:**
```rust
pub struct AddCidsArgs {
    pub new_cids: Vec<String>,
}
```

**Validation:**
- Total CIDs after addition <= 10
- Campaign status is Active
- `authority` matches `publisher_account.authority`

---

#### close_campaign

Closes campaign and returns remaining vault funds to publisher authority.

**Accounts:**
- `campaign_account` (mut, close) - CampaignAccount to close
- `publisher_account` - Parent PublisherAccount
- `authority` (signer, mut) - Receives remaining funds

**Validation:**
- No unclaimed PlayRecords exist for this campaign
- `authority` matches `publisher_account.authority`

---

### Operator Instructions

#### register_node

Registers a new display node with specifications and filters.

**Accounts:**
- `node_account` (init, pda) - NodeAccount to create
- `authority` (signer, mut) - Node operator
- `system_program` - System program

**Args:**
```rust
pub struct RegisterNodeArgs {
    pub node_id: u64,
    pub location: GeoLocation,
    pub screen_size: ScreenSize,
    pub resolution: Resolution,
    pub landmarks: Vec<String>,
    pub content_filters: Vec<String>,
    pub estimated_footfall: u32,
    pub establishment_type: String,
}
```

---

#### update_node_metadata

Updates node location, footfall estimate, or filters.

**Accounts:**
- `node_account` (mut) - NodeAccount to update
- `authority` (signer) - Node authority

**Args:**
```rust
pub struct UpdateNodeMetadataArgs {
    pub location: Option<GeoLocation>,
    pub estimated_footfall: Option<u32>,
    pub content_filters: Option<Vec<String>>,
    pub status: Option<NodeStatus>,
}
```

**Validation:**
- `authority` matches `node_account.authority`

---

#### claim_campaign

Node claims a campaign for display, creating a PlayRecord.

**Accounts:**
- `play_record` (init, pda) - PlayRecord to create
- `campaign_account` (mut) - Campaign being claimed
- `node_account` (mut) - Claiming node
- `authority` (signer) - Node authority
- `system_program` - System program

**Validation:**
- Campaign status is Active
- Node status is Active
- `plays_remaining > 0`
- Vault balance >= `bounty_per_play`
- Node matches campaign `target_filters`
- Campaign tags don't violate node `content_filters`
- No existing unclaimed PlayRecord for this campaign-node pair
- `authority` matches `node_account.authority`

**Side Effects:**
- Decrements `campaign_account.plays_remaining`
- Creates PlayRecord with status Claimed
- Sets 5-minute timeout window

---

#### confirm_play

Node confirms content was displayed, triggering payment.

**Accounts:**
- `play_record` (mut) - PlayRecord to confirm
- `campaign_account` (mut) - Source campaign
- `node_account` (mut) - Destination node
- `authority` (signer) - Node authority

**Validation:**
- `play_record.status == PlayStatus::Claimed`
- Current time <= `claimed_at + 300` (within 5 min)
- `authority` matches `node_account.authority`

**Side Effects:**
- Transfers `bounty_per_play` from CampaignAccount vault to NodeAccount vault
- Updates `play_record.status` to Paid
- Increments `campaign_account.plays_completed`
- Increments `node_account.total_plays`
- Adds to `node_account.total_earnings`

---

### Public Instructions

#### timeout_play

Restores play count for expired claims. Callable by anyone.

**Accounts:**
- `play_record` (mut) - Timed-out PlayRecord
- `campaign_account` (mut) - Associated campaign

**Validation:**
- `play_record.status == PlayStatus::Claimed`
- Current time > `claimed_at + 300` (5 min expired)

**Side Effects:**
- Increments `campaign_account.plays_remaining`
- Updates `play_record.status` to TimedOut

---

## Error Codes

```rust
#[error_code]
pub enum ErrorCode {
    #[msg("Invalid campaign ID")]
    InvalidCampaignId,
    
    #[msg("Too many CIDs (max 10)")]
    TooManyCids,
    
    #[msg("Bounty per play must be > 0")]
    InvalidBounty,
    
    #[msg("Total plays must be > 0")]
    InvalidPlays,
    
    #[msg("Campaign has no plays remaining")]
    NoPlaysRemaining,
    
    #[msg("Insufficient vault balance")]
    InsufficientFunds,
    
    #[msg("Node does not match target filters")]
    TargetMismatch,
    
    #[msg("Content violates node filters")]
    ContentFilterViolation,
    
    #[msg("Existing unclaimed play record")]
    ExistingClaim,
    
    #[msg("Play record not in claimed status")]
    InvalidPlayStatus,
    
    #[msg("Confirmation timeout not yet expired")]
    TimeoutNotExpired,
    
    #[msg("Confirmation timeout has expired")]
    TimeoutExpired,
    
    #[msg("Unauthorized")]
    Unauthorized,
    
    #[msg("Campaign is not active")]
    CampaignNotActive,
    
    #[msg("Node is not active")]
    NodeNotActive,
    
    #[msg("Publisher is not active")]
    PublisherNotActive,
    
    #[msg("Publisher account does not match")]
    PublisherMismatch,
}
```

---

## State Machines

### PublisherAccount States

```
[Created] → Active → Suspended
```

- **Active**: Can create campaigns
- **Suspended**: Cannot create new campaigns

### CampaignAccount States

```
[Created] → Active → Paused ↔ Active → Depleted → Closed
```

- **Active**: Accepting claims from operators
- **Paused**: Temporarily stopped by publisher
- **Depleted**: No plays or funds remaining
- **Closed**: Permanently closed, funds returned

### PlayRecord States

```
[Created] → Claimed → Confirmed → Paid
                   ↘ TimedOut
```

- **Claimed**: Operator claimed but hasn't confirmed
- **Confirmed**: Operator confirmed display
- **Paid**: Bounty transferred to operator
- **TimedOut**: Claim expired without confirmation

---

## Workflows

### Publisher Onboarding & Campaign Creation

```
1. register_publisher() → PublisherAccount created
2. create_campaign(CIDs, targeting) → CampaignAccount created
3. fund_campaign(amount) → SOL deposited to campaign vault
4. Campaign ready for operators to claim
```

### Node Registration & Display Cycle

```
1. register_node(specs, location) → NodeAccount created
2. Query campaigns matching node criteria
3. claim_campaign() → PlayRecord created, 5-min timer starts
4. Display content
5. confirm_play() → Payment transferred, stats updated
```

### Timeout Recovery

```
1. Node claims campaign but fails to confirm within 5 minutes
2. Anyone calls timeout_play()
3. Play count restored to campaign
4. PlayRecord marked as TimedOut
```

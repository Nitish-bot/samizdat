# Account Structures

## PublisherAccount (PDA)

**Seeds**: `["publisher_account", authority.key(), campaign_id.to_le_bytes()]`

Stores campaign information including content CIDs, targeting filters, bounty rates, and vault.

```rust
#[account]
pub struct PublisherAccount {
    /// Campaign creator and authority
    pub authority: Pubkey,
    
    /// Unique campaign identifier
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

## NodeAccount (PDA)

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

## PlayRecord (PDA)

**Seeds**: `["play_record", publisher_account.key(), node_account.key(), claim_timestamp.to_le_bytes()]`

Tracks individual display claims, confirmations, and payment settlements.

```rust
#[account]
pub struct PlayRecord {
    /// Associated campaign
    pub publisher_account: Pubkey,
    
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

## Account Relationships

```
PublisherAccount (1) ────┬──── (N) PlayRecord ──── (N) NodeAccount (1)
                         │
                         └──── Vault (SOL)
                         
NodeAccount (1) ─────────────── Vault (SOL)
```

- One PublisherAccount can have many PlayRecords
- One NodeAccount can have many PlayRecords
- Each PlayRecord links exactly one PublisherAccount to one NodeAccount
- Both PublisherAccount and NodeAccount maintain their own vaults for payment settlement

## PDA Derivation Examples

```rust
// PublisherAccount PDA
let (publisher_pda, bump) = Pubkey::find_program_address(
    &[
        b"publisher_account",
        authority.key().as_ref(),
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
        publisher_account.key().as_ref(),
        node_account.key().as_ref(),
        &claim_timestamp.to_le_bytes(),
    ],
    program_id,
);
```

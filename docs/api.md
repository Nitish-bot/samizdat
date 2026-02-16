# API Reference

## Publisher Instructions

### create_publisher_account

Creates a new campaign with content CIDs and targeting criteria.

**Accounts:**
- `publisher_account` (init, pda) - PublisherAccount to create
- `authority` (signer, mut) - Campaign creator
- `system_program` - System program

**Args:**
```rust
pub struct CreatePublisherAccountArgs {
    pub campaign_id: u64,
    pub cids: Vec<String>,              // 1-10 CIDs
    pub bounty_per_play: u64,           // lamports
    pub total_plays: u64,
    pub tags: Vec<String>,
    pub target_filters: TargetFilters,
}
```

**Validation:**
- `cids.len()` between 1 and 10
- `bounty_per_play > 0`
- `total_plays > 0`

---

### fund_publisher_account

Adds SOL to campaign vault to fund displays.

**Accounts:**
- `publisher_account` (mut) - PublisherAccount to fund
- `authority` (signer, mut) - Campaign authority
- `system_program` - System program

**Args:**
```rust
pub struct FundPublisherAccountArgs {
    pub amount: u64,  // lamports to transfer
}
```

**Validation:**
- `authority` matches `publisher_account.authority`
- Campaign status is Active or Paused
- `amount > 0`

---

### update_publisher_metadata

Updates campaign targeting, tags, or status.

**Accounts:**
- `publisher_account` (mut) - PublisherAccount to update
- `authority` (signer) - Campaign authority

**Args:**
```rust
pub struct UpdatePublisherMetadataArgs {
    pub tags: Option<Vec<String>>,
    pub target_filters: Option<TargetFilters>,
    pub status: Option<CampaignStatus>,
}
```

**Validation:**
- `authority` matches `publisher_account.authority`

---

### add_cids_to_campaign

Adds additional content CIDs to existing campaign.

**Accounts:**
- `publisher_account` (mut) - PublisherAccount to update
- `authority` (signer) - Campaign authority

**Args:**
```rust
pub struct AddCidsArgs {
    pub new_cids: Vec<String>,
}
```

**Validation:**
- Total CIDs after addition <= 10
- Campaign status is Active

---

### close_publisher_account

Closes campaign and returns remaining vault funds to authority.

**Accounts:**
- `publisher_account` (mut, close) - PublisherAccount to close
- `authority` (signer, mut) - Receives remaining funds

**Validation:**
- No unclaimed PlayRecords exist for this campaign

---

## Operator Instructions

### register_node

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

### update_node_metadata

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

### claim_campaign

Node claims a campaign for display, creating a PlayRecord.

**Accounts:**
- `play_record` (init, pda) - PlayRecord to create
- `publisher_account` (mut) - Campaign being claimed
- `node_account` (mut) - Claiming node
- `authority` (signer) - Node authority
- `system_program` - System program

**Validation:**
- Campaign status is Active
- `plays_remaining > 0`
- Vault balance >= `bounty_per_play`
- Node matches campaign `target_filters`
- Campaign tags don't violate node `content_filters`
- No existing unclaimed PlayRecord for this campaign-node pair

**Side Effects:**
- Decrements `publisher_account.plays_remaining`
- Creates PlayRecord with status Claimed
- Sets 5-minute timeout window

---

### confirm_play

Node confirms content was displayed, triggering payment.

**Accounts:**
- `play_record` (mut) - PlayRecord to confirm
- `publisher_account` (mut) - Source campaign
- `node_account` (mut) - Destination node
- `authority` (signer) - Node authority

**Validation:**
- `play_record.status == PlayStatus::Claimed`
- Current time <= `claimed_at + 300` (within 5 min)
- `authority` matches `node_account.authority`

**Side Effects:**
- Transfers `bounty_per_play` from PublisherAccount vault to NodeAccount vault
- Updates `play_record.status` to Paid
- Increments `publisher_account.plays_completed`
- Increments `node_account.total_plays`
- Adds to `node_account.total_earnings`

---

## Public Instructions

### timeout_play

Restores play count for expired claims. Callable by anyone.

**Accounts:**
- `play_record` (mut) - Timed-out PlayRecord
- `publisher_account` (mut) - Associated campaign

**Validation:**
- `play_record.status == PlayStatus::Claimed`
- Current time > `claimed_at + 300` (5 min expired)

**Side Effects:**
- Increments `publisher_account.plays_remaining`
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
}
```

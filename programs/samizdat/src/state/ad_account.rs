use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AdAccount {
    pub authority: Pubkey, // advertiser wallet
    #[max_len(200)]
    pub content_cid: String,
    pub is_active: bool,

    // economics
    pub bounty_per_play: u64, // lamports
    pub max_plays: u64,
    pub play_count: u64,
    pub total_funded: u64,

    // targeting/filter metadata
    pub target_screen_id: Option<Pubkey>, // None => global
    pub tag_mask: u64,                    // off-chain policy matching

    pub bump: u8,
}

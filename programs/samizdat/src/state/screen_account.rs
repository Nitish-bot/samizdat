use crate::state::Dimensions;
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ScreenAccount {
    pub owner_wallet: Pubkey, // payout destination
    pub signing_key: Pubkey,  // renderer signing pubkey
    pub dimensions: Dimensions,

    pub is_active: bool,
    pub last_proof_nonce: u64, // minimal anti-replay helper

    pub bump: u8,
}

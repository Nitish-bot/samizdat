use anchor_lang::prelude::*;

// PDA Seeds
pub const SEED_AD: &[u8] = b"ad";
pub const SEED_SCREEN: &[u8] = b"screen";

// Content Tag Bitmask
// Protocol-level content categories.
// Advertisers SET these on AdAccount.tag_mask.
// Renderers use self defined policy to BLOCK matching bits.
//
// To check overlap:  ad.tag_mask & policy_blocked_mask != 0  → skip
// To check match:    ad.tag_mask & policy_required_mask == policy_required_mask → show
//
// Reserve bit 0–15 for PoC categories.
// Bits 16–63 reserved for future protocol upgrades.

pub const TAG_NONE: u64 = 0;
pub const TAG_CRYPTO: u64 = 1 << 0;
pub const TAG_BETTING: u64 = 1 << 1;
pub const TAG_NSFW: u64 = 1 << 2;
pub const TAG_POLITICAL: u64 = 1 << 3;
pub const TAG_ALCOHOL: u64 = 1 << 4;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub struct Dimensions {
    pub width: u16,
    pub height: u16,
}

use anchor_lang::prelude::*;

pub const PROOF_DOMAIN_V1: &[u8] = b"samizdat:proof:v1";

/// Canonical payload the renderer signs off-chain.
/// The program reconstructs this from instruction inputs
/// and verifies the signature against ScreenAccount.signing_key.
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct ProofPayloadV1 {
    pub ad: Pubkey,
    pub screen: Pubkey,
    pub nonce: u64,
    pub timestamp: i64,
}

impl ProofPayloadV1 {
    /// Serialize domain + payload into a signable message buffer.
    pub fn to_signable_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(PROOF_DOMAIN_V1.len() + 80);
        buf.extend_from_slice(PROOF_DOMAIN_V1);
        buf.extend_from_slice(&self.ad.to_bytes());
        buf.extend_from_slice(&self.screen.to_bytes());
        buf.extend_from_slice(&self.nonce.to_le_bytes());
        buf.extend_from_slice(&self.timestamp.to_le_bytes());
        buf
    }
}

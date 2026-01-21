use anchor_lang::prelude::*;
#[account]
pub struct Proposal {
    pub owner: Pubkey,
    pub title: String,
    pub brief: String,
    pub cate: String,
    pub reference: String,
    pub amount: u64,
    pub agree_votes: u64, // Number of "Agree" votes
    pub disagree_votes: u64,
    pub created_at: i64, // Creation timestamp in seconds
    pub expires_at: i64, // Expiration timestamp in seconds
    pub is_fund_requested: bool,
}

impl Space for Proposal {
    const INIT_SPACE: usize = 32 +        // owner
        (4 + 100) + // title
        (4 + 800) + // brief
        (4 + 50) +  // cate
        (4 + 200) + // reference
        8 +         // amount
        8 +         // agree_votes
        8 +         // disagree_votes
        8 +         // created_at
        8 + // expires_at
        1; // is fund requested
}

#[account]
pub struct UserState {
    pub user: Pubkey,
    pub proposal_count: u64,
}

#[account]
pub struct VoteRecord {
    pub proposal_id: Pubkey, // Linked proposal ID
    pub voter: Pubkey,       // Voter's public key
    pub has_voted: bool,     // Prevent double voting
    pub vote: String,
    pub vote_power: u64,
}
// ========================= STAKE ACCOUNT =========================

#[account]
pub struct StakeAccount {
    pub owner: Pubkey,             // User who owns this stake
    pub staked_amount: u64,        // Total tokens currently staked
    pub pending_rewards: u64,      // Rewards accumulated but not yet claimed
    pub last_stake_time: i64,      // Timestamp of last stake action
    pub pending_unstake: u64,      // Amount requested to unstake
    pub unstake_requested_at: i64, // Timestamp when unstake was requested
}

impl Space for StakeAccount {
    // 8 discriminator
    // 32 owner
    // 8 staked_amount
    // 8 pending_rewards
    // 8 last_stake_time
    // 8 pending_unstake
    // 8 unstake_requested_at
    const INIT_SPACE: usize = 8 + 32 + 8 + 8 + 8 + 8 + 8;
}

// ========================= STATS ACCOUNT =========================

#[account]
pub struct Stats {
    pub total_proposals: u64,       // Total proposals created
    pub active_voters: u64,         // Number of active voters
    pub proposals_passed: u64,      // Successfully passed proposals
    pub treasury_balance: u64,      // Treasury token or SOL balance
    pub total_staked: u64,          // Total amount staked across all users
    pub total_rewards: u64,         // All-time total rewards
    pub total_claimed_rewards: u64, // Sum of claimed rewards for all users
    pub bump: u8,                   // PDA bump
}

impl Space for Stats {
    const INIT_SPACE: usize = 8   // discriminator
        + 8  // total_proposals
        + 8  // active_voters
        + 8  // proposals_passed
        + 8  // treasury_balance
        + 8  // total_staked
        + 8  // total_rewards_distributed
        + 8  // total_unclaimed_rewards
        + 1; // bump
}

// ========================= SETTING ACCOUNT =========================

#[account]
pub struct ConfigAccount {
    pub governance_mint: Pubkey, // allowed mint to stake
}

impl Space for ConfigAccount {
    const INIT_SPACE: usize = 8 // discriminator
    +32 // mint address
    + 256; //Reserve for future
}

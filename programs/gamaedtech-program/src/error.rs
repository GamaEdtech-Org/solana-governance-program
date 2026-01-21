use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized to Action")]
    Unauthorized,

    #[msg("Already voted!")]
    AlreadyVoted,

    #[msg("The Proposal Has Already Expired")]
    ExpiredProposal,

    #[msg("Funds already requested for this proposal.")]
    AlreadyRequested,

    #[msg("Proposal has not passed yet.")]
    ProposalNotPassed,

    #[msg("Proposal voting still active")]
    VotingStillActive,

    #[msg("Proposal Regected")]
    ProposalRejected,

    #[msg("Insufficient Stake")]
    InsufficientStake,

    #[msg("No Stake Power")]
    NoStakePower,

    #[msg("You can only stake the allowed token.")]
    InvalidTokenMint,

    #[msg("Cooldown period still active.")]
    CooldownActive,

    #[msg("Nothing to claim.")]
    NothingToClaim,

    #[msg("Already Unstaking")]
    AlreadyUnstaking,

    #[msg("Math Overflow")]
    MathOverflow,

    #[msg("No Rewards To Claim")]
    NoRewardsToClaim,
}

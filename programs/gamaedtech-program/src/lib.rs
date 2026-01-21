use anchor_lang::prelude::*;
declare_id!("9F77hJsRRXs7vF9UDncZKth2r5wEPgcRkEfyoZDNQ3eK");
const ALLOWED_MINT: &str = "HyXdVykYjcgJwgBmeMmy59QHF4HncsH1TScdH97nqJYW";
const ADMIN: &str = "4SwgW8pqrkCi3AdEqEU9dKGfi2qb4NWWGYkCayugJfrS";

mod instructions;
use instructions::*;
mod error;
mod state;

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;

#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "GamaEdtech Governance",
    project_url: "https://gamatrain.com",
    contacts: "security@gamatrain.com",
    policy: "https://gamatrain.com/security/policy",
    preferred_languages: "en",
    source_code: "https://github.com/GamaEdtech/solana-governance-program"
}

#[program]
pub mod gamaedtech_program {

    use super::*;

    pub fn create_proposal(
        ctx: Context<SubmitProposal>,
        title: String,
        brief: String,
        cate: String,
        reference: String,
        amount: u64,
    ) -> Result<()> {
        proccess_create_proposal(ctx, title, brief, cate, reference, amount)
    }

    pub fn delete_proposal(ctx: Context<DeleteProposal>) -> Result<()> {
        proccess_delete_proposal(ctx)
    }

    pub fn vote(ctx: Context<Vote>, agree: bool) -> Result<()> {
        process_vote(ctx, agree)
    }
    pub fn request_fund(ctx: Context<RequestFund>) -> Result<()> {
        process_request_fund(ctx)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake::process_stake(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        instructions::unstake::process_unstake(ctx, amount)
    }
    pub fn calim_unstake(ctx: Context<ClaimUnstake>) -> Result<()> {
        instructions::unstake::process_claim_unstake(ctx)
    }

    pub fn init_stats(ctx: Context<InitializeStats>) -> Result<()> {
        instructions::init_stats::process_init_stats(ctx)
    }
    pub fn realloc_stats(ctx: Context<ReallocateStats>) -> Result<()> {
        instructions::init_stats::process_realloc_stats(ctx)
    }
    pub fn close_stats(ctx: Context<CloseStats>) -> Result<()> {
        instructions::init_stats::process_close_stats(ctx)
    }

    pub fn claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
        instructions::reward::process_claim_reward(ctx)
    }

    pub fn init_config(ctx: Context<InitializeSetting>, governance_mint: Pubkey) -> Result<()> {
        instructions::config::process_init_config(ctx, governance_mint)
    }
}

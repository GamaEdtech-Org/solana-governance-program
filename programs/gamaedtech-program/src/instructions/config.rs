use crate::state::ConfigAccount;
use anchor_lang::prelude::*;

pub fn process_init_config(ctx: Context<InitializeSetting>, governance_mint: Pubkey) -> Result<()> {
    let config_account = &mut ctx.accounts.config;
    config_account.governance_mint = governance_mint;
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeSetting<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init_if_needed,
        payer=admin,
        seeds=[b"config"],
        bump,
        space= ConfigAccount::INIT_SPACE,
    )]
    pub config: Account<'info, ConfigAccount>,
    pub system_program: Program<'info, System>,
}

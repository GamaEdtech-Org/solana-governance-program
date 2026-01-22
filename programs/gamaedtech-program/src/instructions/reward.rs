use crate::error::ErrorCode;
use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface, TransferChecked};

#[derive(Accounts)]
pub struct ClaimReward<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // User token account (can be Token or Token-2022)
    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.mint == mint.key(),
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    // Reward token account (can be Token or Token-2022)
    #[account(
        mut,
        constraint = reward_token_account.owner == reward_authority.key(),
        constraint = reward_token_account.mint == mint.key(),
    )]
    pub reward_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: reward authority PDA
    #[account(
        seeds = [b"reward-authority"],
        bump
    )]
    pub reward_authority: UncheckedAccount<'info>,

    // Token mint (can be Token or Token-2022)
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        seeds = [b"stake_account", user.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        mut,
        seeds = [b"stats"],
        bump
    )]
    pub stats: Account<'info, Stats>,

    // Token program (TokenInterface supports both Token and Token-2022)
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn process_claim_reward(ctx: Context<ClaimReward>) -> Result<()> {
    let stake_account = &mut ctx.accounts.stake_account;
    let stats = &mut ctx.accounts.stats;

    // Ownership check
    require_keys_eq!(
        stake_account.owner,
        ctx.accounts.user.key(),
        ErrorCode::Unauthorized
    );

    // Ensure user has a pending rewards
    require!(
        stake_account.pending_rewards > 0,
        ErrorCode::NoRewardsToClaim
    );

    // Prepare signer seeds
    let seeds: &[&[u8]] = &[b"reward-authority".as_ref(), &[ctx.bumps.reward_authority]];
    let signer = &[&seeds[..]];

    // Transfer tokens: vault → user
    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        TransferChecked {
            from: ctx.accounts.reward_token_account.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.user_token_account.to_account_info(),
            authority: ctx.accounts.reward_authority.to_account_info(),
        },
        signer,
    );

    token_interface::transfer_checked(
        cpi_ctx,
        stake_account.pending_rewards,
        ctx.accounts.mint.decimals,
    )?;

    stats.total_claimed_rewards = stats
        .total_claimed_rewards
        .checked_add(stake_account.pending_rewards)
        .ok_or(ErrorCode::MathOverflow)?;
    stake_account.pending_rewards = 0;

    Ok(())
}

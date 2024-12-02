use anchor_lang::prelude::*;
use anchor_spl::token::*;

#[program]
pub mod soon_staking {
    use super::*;

    pub fn initialize_stake_pool(ctx: Context<InitializeStakePool>) -> Result<()> {
        let pool = &mut ctx.accounts.stake_pool;
        pool.authority = ctx.accounts.authority.key();
        pool.total_stake = 0;
        pool.msol_mint = ctx.accounts.msol_mint.key();
        Ok(())
    }

    pub fn stake_validator(
        ctx: Context<StakeValidator>,
        amount: u64,
        chain_id: u64
    ) -> Result<()> {
        require!(amount > 0, ErrorCode::InvalidAmount);
        require!(ctx.accounts.validator.is_active, ErrorCode::ValidatorNotActive);

        // transfer like zbtc to stake pool
        anchor_spl::token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                anchor_spl::token::Transfer {
                    from: ctx.accounts.staker_msol.to_account_info(),
                    to: ctx.accounts.pool_msol.to_account_info(),
                    authority: ctx.accounts.staker.to_account_info(),
                },
            ),
            amount,
        )?;

        // Update validator stake
        let validator = &mut ctx.accounts.validator;
        validator.stake += amount;
        validator.chain_id = chain_id;

        // Update pool total
        let pool = &mut ctx.accounts.stake_pool;
        pool.total_stake += amount;

        Ok(())
    }
}

#[account]
pub struct StakePool {
    pub authority: Pubkey,
    pub total_stake: u64,
    pub msol_mint: Pubkey,
}

#[account]
pub struct ValidatorStake {
    pub validator: Pubkey,
    pub stake: u64,
    pub chain_id: u64,
    pub is_active: bool,
}

#[derive(Accounts)]
pub struct InitializeStakePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8 + 32
    )]
    pub stake_pool: Account<'info, StakePool>,
    
    pub msol_mint: Account<'info, Mint>,
    
    #[account(
        init,
        payer = authority,
        token::mint = msol_mint,
        token::authority = stake_pool
    )]
    pub pool_msol: Account<'info, TokenAccount>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct StakeValidator<'info> {
    #[account(
        mut,
    )]
    pub stake_pool: Account<'info, StakePool>,
    
    #[account(
        mut,
    )]
    pub validator: Account<'info, ValidatorStake>,
    
    #[account(
        mut,
        token::mint = stake_pool.msol_mint,
    )]
    pub staker_msol: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        token::mint = stake_pool.msol_mint,
    )]
    pub pool_msol: Account<'info, TokenAccount>,
    
    pub staker: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Validator not active")]
    ValidatorNotActive,
    #[msg("Invalid mint")]
    InvalidMint,
}
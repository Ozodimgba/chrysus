use anchor_lang::prelude::*;

declare_id!("6gswY98TSzTsTWY96ZBtKAVhfsYuwp62kQ1Wgop8BnHf");

#[program]
pub mod chrysus_registry {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let registry = &mut ctx.accounts.registry;
        registry.authority = ctx.accounts.authority.key();
        registry.chain_counter = 0;
        Ok(())
    }

    pub fn register_chain(ctx: Context<RegisterChain>, chain_id: u64) -> Result<()> {
        let chain = &mut ctx.accounts.chain;
        chain.id = chain_id;
        chain.state_root = [0; 32];
        chain.batch_index = 0;
        chain.total_stake = 0;
        chain.is_active = true;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 8
    )]
    pub registry: Account<'info, Registry>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RegisterChain<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 8 + 32 + 8 + 8 + 1
    )]
    pub chain: Account<'info, ChainInfo>,
    pub registry: Account<'info, Registry>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Registry {
    pub authority: Pubkey,
    pub chain_counter: u64,
}

#[account]
pub struct ChainInfo {
    pub id: u64,
    pub state_root: [u32; 32],
    pub batch_index: u64,
    pub total_stake: u64,
    pub is_active: bool,
}

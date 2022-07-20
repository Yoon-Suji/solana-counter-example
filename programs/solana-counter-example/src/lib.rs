use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod solana_counter_example {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub system_program: Program <'info, System>,

    #[account(init, payer = user, space = 16 + 16)]
    pub counter_account: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>
}

#[account]
pub struct Counter {
    pub count: u64
}

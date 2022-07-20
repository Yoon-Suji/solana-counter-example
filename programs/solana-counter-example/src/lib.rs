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

    pub fn increment(ctx: Context<Increment>, count: u64) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        let current_count = counter_account.count;

        // overflow check
        if u64::MAX - current_count >= count {
            counter_account.count = current_count + count;
            return Ok(());
        }
        
        counter_account.count = u64::MAX;
        return Ok(())
    }

    pub fn reset(ctx: Context<Reset>, count: u64) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.count = count;
        return Ok(())
    }
}

// Transaction instructions
#[derive(Accounts)]
pub struct Initialize<'info> {
    pub system_program: Program <'info, System>,

    #[account(init, payer = user, space = 16 + 16)]
    pub counter_account: Account<'info, Counter>,

    #[account(mut)]
    pub user: Signer<'info>
}

// Transaction instructions
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, Counter>
}

// Transaction instruction
#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, Counter>
}

// An account that goes inside a transaction instruction
#[account]
pub struct Counter {
    pub count: u64
}

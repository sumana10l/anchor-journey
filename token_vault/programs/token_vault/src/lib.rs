use anchor_lang::prelude::*;

pub mod errors;
pub mod instructions;
pub mod state;

use errors::*;
use instructions::*;
use state::*;

declare_id!("6sdzRZMrzLVi5pngFW1EuS4V9KUyJ4txoJFtapNvTYES");

#[program]
pub mod token_vault {
    use super::*;

    pub fn initialize_vault(
        ctx: Context<InitializeVault>,
        vault_bump: u8,
        authority_bump: u8,
        reward_rate: u64, 

    ) -> Result<()> {
        instructions::initialize::initialize_vault(ctx, vault_bump, authority_bump, reward_rate)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::deposit(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw::withdraw(ctx, amount)
    }

    pub fn lock_vault(ctx: Context<LockVault>, unlock_timestamp: i64) -> Result<()> {
        instructions::lock::lock_vault(ctx, unlock_timestamp)
    }

    pub fn unlock_vault(ctx: Context<UnlockVault>) -> Result<()> {
        instructions::unlock::unlock_vault(ctx)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        instructions::stake::stake(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        instructions::unstake::unstake(ctx, amount)
    }
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        instructions::claim::claim(ctx)
    }
    pub fn fund_rewards(ctx: Context<FundRewards>, amount: u64) -> Result<()> {
        instructions::fund_rewards::fund_rewards(ctx, amount)
    }
    
}

// anchor build
// solana config set --url devnet
// solana config get
// solana airdrop 2
// solana address [ copy the address and drop some solana from https://faucet.solana.com/]
// solana balance
// anchor deploy

// Program Id: 6sdzRZMrzLVi5pngFW1EuS4V9KUyJ4txoJFtapNvTYES

// Signature: 4rp8xm6rcUTTiuGrm9MqhqfnWtmRwPHKZtkXxjdCM8c6MkxAfm6NYiX5rSbgFJzSwK2sARrxeBLRATA9thANPJyU

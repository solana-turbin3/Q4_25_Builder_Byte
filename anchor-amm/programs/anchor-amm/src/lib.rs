use anchor_lang::prelude::*;

declare_id!("C2eBsV9KV25c9jYSK1MaxLDCgBsoQc4nm5TVP5QpTUmY");

pub mod errors;
pub mod instructions;
pub mod state;

pub use errors::*;
pub use instructions::*;
pub use state::*;

#[program]
pub mod anchor_amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>,
    ) -> Result<()> {
        ctx.accounts.initialize(seed, fee, authority, ctx.bumps)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        ctx.accounts.withdraw(amount, max_x, max_y)
    }

    pub fn swap(ctx: Context<Swap>, is_x: bool, amount_in: u64, min_amount_out: u64) -> Result<()> {
        ctx.accounts.swap(is_x, amount_in, min_amount_out)
    }
}

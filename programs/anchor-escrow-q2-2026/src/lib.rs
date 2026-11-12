pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CZHxvq1XmzEY5Uwh3NtnyPmFMR57Meed8V2xHAJtDpGu");

#[program]
pub mod anchor_escrow_q2_2026 {
    use super::*;

    #[instruction(discriminator = 0)]
    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        deposit: u64,
        receive: u64,
        expiration: i64,
    ) -> Result<()> {
        ctx.accounts.validate_terms(deposit, receive, expiration)?;
        ctx.accounts
            .init_escrow(seed, receive, &ctx.bumps, expiration)?;
        ctx.accounts.deposit(deposit)
    }

    #[instruction(discriminator = 1)]
    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.check_expiration()?;
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()
    }

    #[instruction(discriminator = 2)]
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.check_expiration()?;
        ctx.accounts.refund_and_close_vault()
    }

    #[instruction(discriminator = 3)]
    pub fn update(ctx: Context<Update>, receive: u64, expiration: i64) -> Result<()> {
        ctx.accounts.update_terms(receive, expiration)
    }
}

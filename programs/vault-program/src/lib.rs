use anchor_lang::prelude::*;
use anchor_lang::system_program::{Transfer, transfer};

declare_id!("58F8AwzUTdvMy7VRj3mia9LPgWJ1LgLPYNHYJArEVxfb");

#[program]
pub mod vault_program {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.auth_bump = *ctx.bumps.get("auth").unwrap(); // referring to name of account, not the seed
        ctx.accounts.state.vault_bump = *ctx.bumps.get("vault").unwrap(); // to get the bump, we need to get the bump from the context
        ctx.accounts.state.auth_bump = *ctx.bumps.get("auth").unwrap(); // this line exists because we need to save the bump for the auth account
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };

        let cpi = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            accounts,
        );

        transfer(cpi, amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.owner.to_account_info(),
        };

        let cpi = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            accounts,
        );

        transfer(cpi, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    owner: Signer<'info>, // auth, vault and state are 3 PDAs derived from the owner's public key
    #[account(
        seeds = [b"auth", state.key().as_ref()], // this means only Signer is able to sign on behalf of the vault
        bump,
    )]
    ///CHECK: this is safe
    auth: UncheckedAccount<'info>, // basically this 'auth' is a PDA derived from the signer's public key
    #[account(
        seeds = [b"vault", state.key().as_ref()],
        bump,
    )]
    vault: SystemAccount<'info>, // a regular PDA but owned by the system program
    #[account(
        init,
        payer = owner,
        space = VaultState::LEN,
        seeds = [b"state", owner.key().as_ref()],
        bump,
    )]
    state: Account<'info, VaultState>, // a check to make sure this program owns it
    system_program: Program<'info, System>,
}

impl VaultState {
    const LEN: usize = 8 + 3 * 1;
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    owner: Signer<'info>, // auth, vault and state are 3 PDAs derived from the owner's public key
    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.vault_bump,
    )]
    vault: SystemAccount<'info>, // a regular PDA but owned by the system program
    #[account(
        seeds = [b"state", owner.key().as_ref()],
        bump = state.state_bump,
    )]
    state: Account<'info, VaultState>, // a check to make sure this program owns it
    system_program: Program<'info, System>,
}

#[account]
pub struct VaultState {
    auth_bump: u8,
    vault_bump: u8,
    state_bump: u8,
}

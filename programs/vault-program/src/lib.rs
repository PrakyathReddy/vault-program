use anchor_lang::prelude::*;

declare_id!("58F8AwzUTdvMy7VRj3mia9LPgWJ1LgLPYNHYJArEVxfb");

#[program]
pub mod vault_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

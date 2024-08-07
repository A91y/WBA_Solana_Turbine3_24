use anchor_lang::prelude::*;

declare_id!("6aBMtkCRa7qCCctK4SQkzNLjP8QririQbcqcga2EKGPh");

#[program]
pub mod anchor_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

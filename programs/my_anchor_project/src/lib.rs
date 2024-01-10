use anchor_lang::prelude::*;

declare_id!("3AWTmg7RU2vM8MZG64veUtthB4EVybSN2rHrHuZ6VWtX");

#[program]
pub mod my_anchor_project {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Function initialize called");


        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

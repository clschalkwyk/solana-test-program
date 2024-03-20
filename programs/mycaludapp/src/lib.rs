use anchor_lang::prelude::*;

declare_id!("3c5NhCgPTfPXPwUtY62a4wTZLZFZs4nhnXG5EJBfANJN");

#[program]
pub mod mycaludapp {
    use anchor_lang::solana_program::entrypoint::ProgramResult;
    use super::*;
    pub fn create(_ctx: Context<Create>, init_message: String) -> ProgramResult{
        let calculator = &mut _ctx.accounts.calculator;
        calculator.greeting = init_message;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info>{
    #[account(init, payer=user, space=264)]
    pub calculator: Account<'info, Calculator>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Calculator{
    pub greeting: String,
    pub result: i64,
    pub remainder: i64
}
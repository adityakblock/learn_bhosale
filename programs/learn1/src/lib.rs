use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use spl_token::*;

declare_id!("ErGamnm2JKDLxu4cFdhqHjJjYBxzCwvY1G8vfNWKyfvz");

#[program]
pub mod learn1 {
    use super::*;
    pub fn new(ctx: Context<Initialize>) -> ProgramResult {
        let d = &mut ctx.accounts.data;
        d.increment = 20;

        msg!("Done successfull");
        Ok(())
    }

    pub fn view_data(ctx: Context<View>) -> ProgramResult {

        msg!("The value is {}", ctx.accounts.data.increment);

        Ok(())
    }

    pub fn increment_data(ctx: Context<Add>, val:u64) -> ProgramResult {
        let d = &mut ctx.accounts.data;
        d.increment = d.increment + val;
        msg!("Done");

        Ok(())
    }
}

#[account]
pub struct Data {
    pub increment: u64
}

#[derive(Accounts)]
pub struct Initialize<'info> {

    #[account(
        init,
        seeds = [b"meta_data".as_ref()],
        bump,
        space = 100,
        payer = deployer
    )]
    pub data : Account<'info, Data>,

    #[account(mut)]
    pub deployer: Signer<'info>,


    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,

}



#[derive(Accounts)]
pub struct View<'info> {

    #[account(
        seeds = [b"meta_data".as_ref()],
        bump,
    )]
    pub data : Account<'info, Data>,

    #[account(mut)]
    pub viewer: Signer<'info>,


    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,

}


#[derive(Accounts)]
pub struct Add<'info> {

    #[account(
        mut,
        seeds = [b"meta_data".as_ref()],
        bump,
    )]
    pub data : Account<'info, Data>,

    #[account(mut)]
    pub adder: Signer<'info>,


    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub clock: Sysvar<'info, Clock>,

}

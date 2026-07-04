use anchor_lang::prelude::*;
use anchor_spl::{ associated_token::AssociatedToken, 
    token_interface::{Mint, TokenInterface, TokenAccount}};

use crate::{AmmError, Amm};


// swap struct contains:
//          amm account
//          user token_to_swap =>must match any of the two token in amm acc
//          check if tokens are available  in the vaults or not must be greater than zero
//          

pub fn swap_token_a(ctx:& Context<SwapToken>,amount:u64){
    
}

pub fn swap_token_b(ctx:& Context<SwapToken>,amount:u64){

}

pub  fn swap_token(ctx:Context<SwapToken>, amount:u64)->Result<()>{
    
    let user_token_address=ctx.accounts.user_token.key();

    let token_a_add=ctx.accounts.token_a.key();

    let token_b_add=ctx.accounts.token_b.key();

    require!(user_token_address==token_a_add||user_token_address==token_b_add,AmmError::InvalidTokenSwapAddress);
    
    require!(amount==0,AmmError::InvalidTokenSwapAmount);



    let amm=&ctx.accounts.amm_acc;

    if user_token_address==token_a_add{
        swap_token_a(&ctx , amount);
    }else{
        swap_token_b(&ctx, amount);
    }
    


    Ok(())
}

#[derive(Accounts)]
pub struct SwapToken<'info>{
    #[account(mut)]
    pub amm_acc:Account<'info,Amm>,

    #[account(mut)]
    pub user:Signer<'info>,

    #[account(mint::token_program=token_program)]
    user_token:InterfaceAccount<'info,Mint>,

    #[account(mint::token_program=token_program,
        mint::authority=amm_acc,
        address=amm_acc.token_a
    )]
    pub token_a:InterfaceAccount<'info,Mint>,

    #[account(mint::token_program=token_program,
        mint::authority=amm_acc,
        address=amm_acc.token_b
    )]
    pub token_b:InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint=token_a,
        associated_token::authority=amm_acc,
        associated_token::token_program=token_program
    )]
    pub vault_a:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=token_b,
        associated_token::authority=amm_acc,
        associated_token::token_program=token_program
    )]
    pub vault_b:Box<InterfaceAccount<'info,TokenAccount>>,
    
    #[account(init_if_needed,
        payer=user,
        associated_token::mint=user_token,
        associated_token::authority=user,
        associated_token::token_program=token_program
    )]
    pub user_ata:Box<InterfaceAccount<'info,TokenAccount>>,

    pub associated_token_program:Program<'info,AssociatedToken>,

    pub token_program:Interface<'info,TokenInterface>,

    pub system_program:Program<'info,System>
}

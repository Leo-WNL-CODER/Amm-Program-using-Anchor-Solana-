use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::Amm;

pub fn intialize(ctx:Context<Initialize>)->Result<()>{
    require_keys_neq!(
        ctx.accounts.token_a.key(),
        ctx.accounts.token_b.key()
    );
    let amm_acc=&mut ctx.accounts.amm_acc;
    amm_acc.token_a=ctx.accounts.token_a.key();
    amm_acc.token_b=ctx.accounts.token_b.key();
    amm_acc.bump= ctx.bumps.amm_acc;
    amm_acc.lp_token=ctx.accounts.lp_token.key();
    amm_acc.token_a_decimal=ctx.accounts.token_a.decimals;
    amm_acc.token_b_decimal=ctx.accounts.token_b.decimals;
    Ok(())
}



#[derive(Accounts)]
pub struct Initialize<'info>{
    #[account(mut)]
    pub signer:Signer<'info>,

    #[account(init,
    payer=signer,
    space=8+Amm::INIT_SPACE,
    seeds=[b"amm",token_a.key().as_ref(),token_b.key().as_ref()],
    bump)
    ]
    pub amm_acc:Account<'info,Amm>,
    
    #[account(mint::token_program=token_program)]
    pub token_a:InterfaceAccount<'info,Mint>,

    #[account(mint::token_program=token_program)]
    pub token_b:InterfaceAccount<'info,Mint>,

    #[account(mint::token_program=token_program,
    mint::authority=amm_acc)]
    pub lp_token:InterfaceAccount<'info,Mint>,

    #[account(
        init,
        payer=signer,
        associated_token::mint=token_a,
        associated_token::authority=amm_acc,
        associated_token::token_program=token_program,
    )]
    pub vault_a:InterfaceAccount<'info,TokenAccount>,

    #[account(
        init,
        payer=signer,
        associated_token::mint=token_b,
        associated_token::authority=amm_acc,
        associated_token::token_program=token_program,
    )]
    pub vault_b:InterfaceAccount<'info,TokenAccount>,

    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Interface<'info,TokenInterface>,
    pub system_program:Program<'info,System>
}


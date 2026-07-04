use anchor_lang::{prelude::*, };
use anchor_spl::{associated_token::AssociatedToken, token::{ Burn, TransferChecked, burn, transfer_checked}, token_2022::MintTo, token_interface::{self, Mint, TokenAccount, TokenInterface}};

use crate::{Amm, AmmError};



pub fn withdraw_liquidity(ctx:Context<WithdrawLiquidity>,lp_w_token:u64)->Result<()>{
    let amm=&mut ctx.accounts.amm_acc;

    let curr_lp_token=ctx.accounts.lp_user_ata.amount;

    require_gte!(curr_lp_token,lp_w_token);
    let vault_a_token=ctx.accounts.vault_a.amount;
    
    let vault_b_token=ctx.accounts.vault_b.amount;

    let lp_token_sup=ctx.accounts.lp_token.supply;

    require!(lp_token_sup > 0, AmmError::InvalidLPSupply);

    //calculating the repective
    // tokens that have to be transfered to lp

    let tnsfr_token_a=(((lp_w_token as u128)*(vault_a_token as u128))/(lp_token_sup as u128)) as u64;
    
    let tnsfr_token_b=(((lp_w_token as u128)*(vault_b_token as u128))/(lp_token_sup as u128)) as u64;

    let token_program=ctx.accounts.token_program.to_account_info();

    let token_a_decimal=amm.token_a_decimal;
    let token_b_decimal=amm.token_b_decimal;
    // let lp_decimal=amm.lp_decimal;

    let token_a_key=amm.token_a;
    let token_b_key=amm.token_b;

    let signer_seeds: &[&[&[u8]]]=&[&[b"amm",
    token_a_key.as_ref(),token_b_key.as_ref(),&[ctx.bumps.amm_acc]]];

    //Burning the   LP token
    let burn_t=Burn{
        mint:ctx.accounts.lp_token.to_account_info(),
        from:ctx.accounts.lp_user_ata.to_account_info(),
        authority:ctx.accounts.lp_user.to_account_info()
    };

    let burn_cpi=CpiContext::new(token_program.key()
    , burn_t);

    burn(burn_cpi, lp_w_token)?;



    let cpi_a=CpiContext::new_with_signer(token_program.key(),
     TransferChecked{
        from:ctx.accounts.vault_a.to_account_info(),
        to:ctx.accounts.lp_user_token_a.to_account_info(),
        mint:ctx.accounts.token_a.to_account_info(),
        authority:ctx.accounts.amm_acc.to_account_info()
    }, signer_seeds);

    transfer_checked(cpi_a, tnsfr_token_a, token_a_decimal)?;
    
    let cpi_b=CpiContext::new_with_signer(token_program.key(),
     TransferChecked{
        from:ctx.accounts.vault_b.to_account_info(),
        to:ctx.accounts.lp_user_token_b.to_account_info(),
        mint:ctx.accounts.token_b.to_account_info(),
        authority:ctx.accounts.amm_acc.to_account_info()
    }, signer_seeds);

    transfer_checked(cpi_b, tnsfr_token_b, token_b_decimal)?;

    Ok(())
}


#[derive(Accounts)]
pub struct WithdrawLiquidity<'info>{

    #[account(mut)]
    pub lp_user:Signer<'info>,

    #[account(
        mut,
        seeds=[b"amm",token_a.key().as_ref(),token_b.key().as_ref()],
        bump
    )]
    pub amm_acc:Account<'info,Amm>,

    #[account(mint::token_program=token_program,
    address=amm_acc.token_a)]
    pub token_a:InterfaceAccount<'info,Mint>,

    #[account(mint::token_program=token_program,
        address=amm_acc.token_b)]
    pub token_b:InterfaceAccount<'info,Mint>,

    #[account(mint::token_program=token_program,
        address=amm_acc.lp_token,
        mint::authority=amm_acc
    )]
    pub lp_token:InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint=token_a,
        associated_token::token_program=token_program,
        associated_token::authority=amm_acc
    )]
    pub vault_a:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=token_b,
        associated_token::token_program=token_program,
        associated_token::authority=amm_acc
    )]
    pub vault_b:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=lp_token,
        associated_token::token_program=token_program,
        associated_token::authority=lp_user
    )]
    pub lp_user_ata:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=token_a,
        associated_token::token_program=token_program,
        associated_token::authority=lp_user
    )]
    pub lp_user_token_a:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=token_b,
        associated_token::token_program=token_program,
        associated_token::authority=lp_user
    )]
    pub lp_user_token_b:InterfaceAccount<'info,TokenAccount>,

    pub associated_token_program:Program<'info,AssociatedToken>,
    pub token_program:Interface<'info,TokenInterface>,
    // pub system_program:Program<'info,System>
}
use anchor_lang::{prelude::*, system_program::transfer};
use anchor_spl::{ associated_token::AssociatedToken, 
    token::{TransferChecked, transfer_checked},
    token_interface::{Mint, TokenInterface, TokenAccount}};

use crate::{AmmError, Amm};




pub  fn swap_token(ctx:Context<SwapToken>, amount:u64)->Result<()>{
    
    let user_token_address=ctx.accounts.user_token.key();

    let token_a_add: Pubkey=ctx.accounts.token_a.key();

    let token_b_add: Pubkey=ctx.accounts.token_b.key();

    let user_recieve_token_add=ctx.accounts.user_recieve_token.key();

    require!(user_token_address==token_a_add||user_token_address==token_b_add,AmmError::InvalidTokenSwapAddress);
    require!(user_recieve_token_add==token_a_add||user_recieve_token_add==token_b_add,
    AmmError::InvalidTokenSwapAddress);
    require!((user_recieve_token_add!=user_token_address),
    AmmError::IdenticalTokenAddress);
    require!(amount>0,AmmError::InvalidTokenSwapAmount);


    let amm=&ctx.accounts.amm_acc;

    let fee =
    (amount as u128)
    * (amm.swap_fee as u128)
    / (amm.swap_fee_decimal as u128);

    let new_user_amount:u64=amount-fee as u64;

    //have to add checks mentioned below
    //Trading fees---done 
    // User-specified slippage limits
    // Liquidity checks
    // Safe integer arithmetic

    //transfering the amount of tokens from user ata to vault
    let (swap_vault,send_vault,mint_recieve,
        mint_send,swap_decimal,
        send_decimal,prev_swap_amount,swap_add,send_token_add) = {
        if user_token_address==token_a_add{
            (
            ctx.accounts.vault_a.to_account_info(),
            ctx.accounts.vault_b.to_account_info(),
            ctx.accounts.token_a.to_account_info(),
            ctx.accounts.token_b.to_account_info(),
            amm.token_a_decimal,
            amm.token_b_decimal,
            ctx.accounts.vault_a.amount,
            token_a_add,
            token_b_add)
        }else{
            (ctx.accounts.vault_b.to_account_info(),
            ctx.accounts.vault_a.to_account_info(),
            ctx.accounts.token_b.to_account_info(),
            ctx.accounts.token_a.to_account_info(),
            amm.token_b_decimal,
            amm.token_a_decimal,
            ctx.accounts.vault_b.amount,
            token_b_add,
            token_a_add)
        }
    };


    
    //calculate the amount of tokens that to be deducted from another 
    //vault and send them to send token add user ata  
    let cur_send_amount=
    if user_token_address==token_a_add{
        ctx.accounts.vault_b.amount
    }else{
        ctx.accounts.vault_a.amount
    };

    let k=((prev_swap_amount as u128)*(cur_send_amount as u128));

    let total_swap_amount=(prev_swap_amount+new_user_amount);
    let new_send_amount=(k)/(total_swap_amount as u128);


    let amount_to_send_user=(cur_send_amount as u128)-new_send_amount;
    
    // require!(new_send_amount>=0,AmmError::NoTokensAvailable);

    require!((new_send_amount*total_swap_amount as u128)>=k,AmmError::NoTokensAvailable);
    require!(amount_to_send_user>0 , AmmError::SwapNotPossible);


    let from=ctx.accounts.user_swap_ata.to_account_info();

    let cpi_from_user_to_vault=CpiContext::new(ctx.accounts.token_program.key(),
     TransferChecked{
        from,
        to:swap_vault,
        mint:mint_recieve,
        authority:ctx.accounts.user.to_account_info()
     });

    transfer_checked(cpi_from_user_to_vault, amount,swap_decimal)?;
    
    // transfering the required tokens to user ata

    let (t_a,t_b)=if user_token_address==token_a_add{
        (token_a_add,token_b_add)
    }else{
        (token_b_add,token_a_add)

    };
    let signer_seeds: &[&[&[u8]]]=&[&[b"amm",t_a.as_ref(),
    t_b.as_ref(),
    &[ctx.bumps.amm_acc]]];
    
    let cpi_from_vault_to_user=CpiContext::new_with_signer(ctx.accounts.token_program.key(),
     TransferChecked{
        from:send_vault,
        to:ctx.accounts.user_recieve_ata.to_account_info(),
        mint:mint_send,
        authority:ctx.accounts.amm_acc.to_account_info()
     },signer_seeds);

    transfer_checked(cpi_from_vault_to_user, 
    amount_to_send_user as u64,
    send_decimal)?;


    // will add slippage check at last
    Ok(())
}

#[derive(Accounts)]
pub struct SwapToken<'info>{
    #[account(mut,
        seeds=[b"amm",token_a.key().as_ref(),token_b.key().as_ref()],
        bump
    )]
    pub amm_acc:Account<'info,Amm>,

    #[account(mut)]
    pub user:Signer<'info>,

    #[account(mint::token_program=token_program)]
    user_token:InterfaceAccount<'info,Mint>,

    #[account(mint::token_program=token_program)]
    user_recieve_token:InterfaceAccount<'info,Mint>,

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
    
    #[account(
        mut,
        associated_token::mint=user_token,
        associated_token::authority=user,
        associated_token::token_program=token_program
    )]
    pub user_swap_ata:Box<InterfaceAccount<'info,TokenAccount>>,
    #[account(
        init_if_needed,
        payer=user,
        associated_token::mint=user_recieve_token,
        associated_token::authority=user,
        associated_token::token_program=token_program
    )]
    pub user_recieve_ata:Box<InterfaceAccount<'info,TokenAccount>>,

    pub associated_token_program:Program<'info,AssociatedToken>,

    pub token_program:Interface<'info,TokenInterface>,

    pub system_program:Program<'info,System>
}

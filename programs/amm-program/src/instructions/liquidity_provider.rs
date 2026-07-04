use anchor_lang::{prelude::*, };
use anchor_spl::{associated_token::AssociatedToken, token::{ TransferChecked, transfer_checked}, token_2022::MintTo, token_interface::{self, Mint, TokenAccount, TokenInterface}};

use crate::Amm;



//this fn takes the token from the LP and adds it to the pool
//the amount of tokens that user is providing must be gt than 0
//here we divide the function in two parts-
//          -first when the account is initalized and token counts are 0
//          -second when the account already has some tokens
pub fn provide_liquidity(ctx:Context<LiquidityProvider>,lp_a:u64,lp_b:u64)->Result<()>{
    
    require_gt!(lp_a,0);
    require_gt!(lp_b,0);

    let  amm_acc=&mut ctx.accounts.amm_acc;

    let a_count=ctx.accounts.vault_a.amount;
    let b_count=ctx.accounts.vault_b.amount;
    
    let mut lp_token_amt:u64=0;

    if a_count==0 && b_count==0{
       
        lp_token_amt=((lp_a as u128*lp_b as u128) as f32).sqrt() as u64;

    }else{

        let required_b=((b_count as u128)*(lp_a as u128))/(a_count as u128);

        require_eq!(required_b as u64,lp_b);

        lp_token_amt=(((lp_a as u128)*(ctx.accounts.lp_token.supply as u128))/(a_count as u128))as u64;

    } 
    let token_program=ctx.accounts.token_program.to_account_info();


    let dest_vault_a=ctx.accounts.vault_a.to_account_info();
    let dest_vault_b=ctx.accounts.vault_b.to_account_info();



    let cpi_a=CpiContext::new(
        token_program.key(),
        TransferChecked{
            from:ctx.accounts.lp_user_token_a_ata.to_account_info(),
            mint:ctx.accounts.token_a.to_account_info(),
            to:dest_vault_a,
            authority:ctx.accounts.lp_user.to_account_info()
        }   
    );

    let cpi_b=CpiContext::new(
        token_program.key(),
        TransferChecked{
            from:ctx.accounts.lp_user_token_b_ata.to_account_info(),
            mint:ctx.accounts.token_b.to_account_info(),
            to:dest_vault_b,
            authority:ctx.accounts.lp_user.to_account_info()
        }   
    );

    transfer_checked(cpi_a, lp_a , ctx.accounts.token_a.decimals)?;

    transfer_checked(cpi_b, lp_b , ctx.accounts.token_b.decimals)?;
    
    let token_a_key=amm_acc.token_a;
    let token_b_key=amm_acc.token_b;

    let signer_seeds: &[&[&[u8]]]=&[&[b"amm",
    token_a_key.as_ref(),token_b_key.as_ref(),&[ctx.bumps.amm_acc]]];

    let mint_lp_token=MintTo{
        mint:ctx.accounts.lp_token.to_account_info(),
        to:ctx.accounts.user_lp_token_ata.to_account_info(),
        authority:ctx.accounts.amm_acc.to_account_info()
    };

    let mint_cpi = CpiContext::new_with_signer(token_program.key(),
         mint_lp_token,signer_seeds);

    token_interface::mint_to(mint_cpi,lp_token_amt)?;
    

    Ok(())
}

#[derive(Accounts)]
pub struct LiquidityProvider<'info>{

    #[account(mut)]
    pub lp_user:Signer<'info>,
   
    #[account(mut,
        seeds=[b"amm",token_a.key().as_ref(),token_b.key().as_ref()],
        bump)
        ]
    pub amm_acc:Account<'info,Amm>,

    #[account(mint::token_program=token_program,
    address=amm_acc.token_a)]
    pub token_a:InterfaceAccount<'info,Mint>,

    #[account(mint::token_program=token_program,
        address=amm_acc.token_b)]
    pub token_b:InterfaceAccount<'info,Mint>,

    #[account(mint::token_program=token_program,
        address=amm_acc.lp_token,
    mint::authority=amm_acc)]
    pub lp_token:InterfaceAccount<'info,Mint>,

    #[account(
        mut,
        associated_token::mint=token_a,
        associated_token::authority=amm_acc,
        associated_token::token_program=token_program,
    )]
    pub vault_a:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint=token_b,
        associated_token::authority=amm_acc,
        associated_token::token_program=token_program,
    )]
    pub vault_b:InterfaceAccount<'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer=lp_user,
        associated_token::mint=lp_token,
        associated_token::authority=lp_user,
        associated_token::token_program=token_program
    )]
    pub user_lp_token_ata:Box<InterfaceAccount<'info,TokenAccount>>,

    
    #[account(mut,
        associated_token::mint=token_a,
        associated_token::authority=lp_user,
        associated_token::token_program=token_program
    )]
    pub lp_user_token_a_ata:Box<InterfaceAccount<'info,TokenAccount>>,

    #[account(mut,
        associated_token::mint=token_b,
        associated_token::authority=lp_user,
        associated_token::token_program=token_program
    )]
    pub lp_user_token_b_ata:Box<InterfaceAccount<'info,TokenAccount>>,


    pub associated_token_program:Program<'info,AssociatedToken>,
    
    pub token_program:Interface<'info,TokenInterface>,

    pub system_program:Program<'info,System>
}
use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Amm {
    pub lp_token:Pubkey,
    pub token_a:Pubkey,
    pub token_b:Pubkey,
    pub token_a_decimal:u8,
    pub token_b_decimal:u8,
    pub lp_decimal:u8,
    pub bump:u8
}

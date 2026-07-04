use anchor_lang::prelude::*;

#[error_code]
pub enum AmmError {
    #[msg("Only the counter authority can update this counter")]
    Unauthorized,
    #[msg("Counter has reached the maximum value")]
    CounterOverflow,
    #[msg("LP supply is 0.")]
    InvalidLPSupply,
    #[msg("User Token Address is invalid.Unable to Swap.")]
    InvalidTokenSwapAddress,
    #[msg("Invalid Swap Amount.")]
    InvalidTokenSwapAmount

}

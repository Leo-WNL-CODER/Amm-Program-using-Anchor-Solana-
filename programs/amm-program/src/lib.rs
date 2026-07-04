#![allow(unused)]
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

// pub use instructions::*;
pub use state::*;
pub use instructions::*;
pub use error::*;
declare_id!("6JFjLpzdjyS1fsvqgEG9SuMMTBNcpU21ky48AxieF7N4");

#[program]
pub mod amm_program {
    use super::*;

    
}

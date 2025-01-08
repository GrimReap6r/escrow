pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("BW3T7qvk1DH9Yg76WjRRcLbxM1jQht4sLCqKM8FeseWj");

#[program]
pub mod anchor_escrow {

    use super::*;

    pub fn make(context: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        instructions::make::send_offered_tokens_to_vault(&context, deposit)?;
        instructions::make::save_offer(context, seed, receive)
    }

    pub fn take(context: Context<Take>) -> Result<()> {
        instructions::take::send_wanted_tokens_to_maker(&context)?;
        instructions::take::withdraw_and_close_vault(context)
    }

    pub fn refund(context: Context<Refund>) -> Result<()> {
        instructions::refund::refund_and_close_vault(context)
    }
}

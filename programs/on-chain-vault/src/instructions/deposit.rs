//-------------------------------------------------------------------------------
///
/// TASK: Implement the deposit functionality for the on-chain vault
/// 
/// Requirements:
/// - Verify that the user has enough balance to deposit
/// - Verify that the vault is not locked
/// - Transfer lamports from user to vault using CPI (Cross-Program Invocation)
/// - Emit a deposit event after successful transfer
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use crate::state::Vault;
use crate::errors::VaultError;
use crate::events::DepositEvent;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>, // mutable because this user is making the transfer to the vault
    #[account(mut)]
    pub vault: Account<'info, Vault>, //mutable because the vault balance will change too
    pub system_program: Program<'info, System> // needed because the user making this transfer may not be the vault owner, hence a CPI is needed
}

pub fn _deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    // retrieve the user and vault accounts
    

    // check if the user has more lamports balance

    // verify that the vault is not locked

    // transfer the lamports using CPI to invoke system program

    // emit success response

    // return result
}
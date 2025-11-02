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
    let user = &mut ctx.accounts.user;
    let vault = &mut ctx.accounts.vault;

    // check if the user has more lamports balance
    require!(user.to_account_info().lamports() >= amount, VaultError::InsufficientBalance);

    // verify that the vault is not locked
    require!(!vault.locked, VaultError::VaultLocked);

    // transfer the lamports using CPI to invoke system program
    // create the transfer instruction (sys prog): it takes the from pubkey, the to pubkey and the amount of lamports
    let transfer_instruction = transfer(
        user.key(),
        vault.key(),
        amount
    );

    // invoke the instruction, provide an array of all accounts involved in the transaction placing the signer first
    invoke(
        &transfer_instruction,
        &[
            user.to_account_info(), // the user signs the transaction
            vault.to_account_info(),
            ctx.accounts.system_program.to_account_info()
        ]
    )?;

    // emit success response

    // return result
}
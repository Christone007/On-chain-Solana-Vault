//-------------------------------------------------------------------------------
use crate::errors::VaultError;
use crate::events::WithdrawEvent;
use crate::state::Vault;
///
/// TASK: Implement the withdraw functionality for the on-chain vault
///
/// Requirements:
/// - Verify that the vault is not locked
/// - Verify that the vault has enough balance to withdraw
/// - Transfer lamports from vault to vault authority
/// - Emit a withdraw event after successful transfer
///
///-------------------------------------------------------------------------------
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    // TODO: Add required accounts and constraints
    #[account(mut)]
    pub vault_authority: Signer<'info>,
    #[account(mut, has_one = vault_authority)]
    pub vault: Account<'info, Vault>,
}

pub fn _withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    // get the vault and authority
    let vault = &mut ctx.accounts.vault;
    let authority = &mut ctx.accounts.vault_authority;

    // verify vault is not locked
    require!(!vault.locked, VaultError::VaultLocked);

    // calculate the minimum balance required for Rent
    let rent_minimum = Rent::get()?.minimum_balance(vault.to_account_info().data_len());
    let max_withdraw = vault.to_account_info().lamports() - rent_minimum;

    

    // verify vault balance is >= amount
    require!(
        amount <= max_withdraw,
        VaultError::InsufficientBalance
    );

    // since vault is owned by the program, i don't need system_program for transfer
    // deduct from vault balance and add to authority balance
    **ctx.accounts.vault.to_account_info().try_borrow_mut_lamports()? -= amount;
    **ctx.accounts.vault_authority.try_borrow_mut_lamports()? += amount;

    // emit message
    emit!(WithdrawEvent {
            amount,
            vault_authority: ctx.accounts.vault_authority.key(),
            vault: ctx.accounts.vault.key(),
        }

    );

    // return result
    Ok(())
}

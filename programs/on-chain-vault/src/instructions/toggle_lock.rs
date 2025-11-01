//-------------------------------------------------------------------------------
///
/// TASK: Implement the toggle lock functionality for the on-chain vault
/// 
/// Requirements:
/// - Toggle the locked state of the vault (locked becomes unlocked, unlocked becomes locked)
/// - Only the vault authority should be able to toggle the lock
/// - Emit a toggle lock event after successful state change
/// 
///-------------------------------------------------------------------------------

use anchor_lang::prelude::*;
use crate::state::Vault;
use crate::events::ToggleLockEvent;

#[derive(Accounts)]
pub struct ToggleLock<'info> {
    pub vault_authority: Signer<'info>,
    #[account(mut)]
    pub vault: Account<'info, Vault>
}

pub fn _toggle_lock(ctx: Context<ToggleLock>) -> Result<()> {
    // bring in the vault
    let vault = &mut ctx.accounts.vault;
    let authority = &mut ctx.accounts.vault_authority;

    // toggle its lock value
    if vault.vault_authority == authority.key() {
        vault.locked = !vault.locked;
        
        // emit event
        emit!(ToggleLockEvent {
            vault: vault.key(),
            vault_authority: authority.key(),
            locked: vault.locked,
        });
    }
    
    // retrun Result
    Ok(())
}
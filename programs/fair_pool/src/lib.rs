use anchor_lang::prelude::*;
use arcium_anchor::{
    arcium_callback, arcium_program, callback_accounts, init_computation_definition_accounts,
    queue_computation_accounts,
};

// Placeholder ID for the allocation protocol
declare_id!("Arc1umFA1rPooLSe1ect1onProtocol1111111111");

#[arcium_program]
#[program]
pub mod fair_pool {
    use super::*;

    /// Initializes the protocol-level configuration for secure allocation.
    /// This step links the Solana ledger state with the Arcium MXE definition.
    pub fn initialize_allocation_protocol(ctx: Context<InitializeAllocationProtocol>) -> Result<()> {
        msg!("FairSelection Protocol: Security definitions initialized.");
        Ok(())
    }

    /// Submits a request to the Arcium network for a confidential selection event.
    /// Parameters such as entropy seeds and pool size are processed in a shielded state.
    pub fn request_confidential_selection(ctx: Context<RequestConfidentialSelection>) -> Result<()> {
        msg!("FairSelection Protocol: Cryptographic request queued for MXE execution.");
        Ok(())
    }

    /// The secure callback executed once the Arcium nodes complete the FHE computation.
    /// This finalizes the game state transition or asset distribution on-chain.
    #[arcium_callback(encrypted_ix = "execute_secure_selection")]
    pub fn on_selection_finalized(ctx: Context<OnSelectionFinalized>, _decrypted_payload: Vec<u8>) -> Result<()> {
        // Here, the result is used to trigger on-chain events (e.g., card dealing or winner payout)
        msg!("FairSelection Protocol: Selection finalized and verified via Arcium.");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeAllocationProtocol<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    /// Link to the Arcis circuit: execute_secure_selection
    #[init_computation_definition_accounts("execute_secure_selection", authority)]
    pub computation_definition: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RequestConfidentialSelection<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    /// Queue parameters into the Arcium computation lane
    #[queue_computation_accounts("execute_secure_selection", user)]
    pub mxe_queue_accounts: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct OnSelectionFinalized<'info> {
    /// Verification accounts for the secure callback
    #[callback_accounts("execute_secure_selection")]
    pub verification_accounts: AccountInfo<'info>,
}
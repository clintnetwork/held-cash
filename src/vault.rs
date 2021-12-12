use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct VaultDetails {
    // Owner's public key
    pub owner: Pubkey,

    // Multisig cosigners
    pub cosigners: [Pubkey; 2],

    // Number of required signatures
    pub requires: i32,
}

// Create a new vault
pub fn create_vault(
    program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Success to create vault");

    Ok(())
}
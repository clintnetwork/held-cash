use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Running held-cash");

    // The program doesn't received any instruction data
    if instruction_data.len() == 0 {
        return Err(ProgramError::InvalidInstructionData);
    }

    // We process the instructions
    if instruction_data[0] == 0 {
        return crate::vault::create_vault(program_id, accounts, &instruction_data[1..instruction_data.len()]);
    } else {
        msg!("This instruction is not valid");
        return Err(ProgramError::InvalidInstructionData);
    }
}

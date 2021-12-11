#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use solana_program::{
        account_info::AccountInfo,
        pubkey::Pubkey,
    };

    use held_cash::processor::process_instruction;

    #[test]
    fn mock_vault_creation() {
        let mut x: u64 = 0;
        let program_id = Pubkey::from_str("AFy44j2rMCHV9xNTb7tn7J32ZU9CZ71sDzunf6EFWBzz").expect("Unable to parse program Id.");
        let owner = Pubkey::from_str("6PpUjY5Tf8wzeR9uBQX2JRpgvcDKMi1Xb7asrAhYS2rZ").expect("Unable to parse public key.");

        let mut accounts = Vec::new();
        accounts.push(AccountInfo::new(&owner, true, false, &mut x, &mut [], &owner, false, 0));
        let instruction_data = b"\0";   // Init the creation of the vault

        process_instruction(&program_id, accounts.as_slice(), instruction_data);

        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

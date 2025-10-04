use borsh::{BorshDeserialize, BorshSerialize};
use mpl_core::instructions::UpdateV1CpiBuilder;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::utils::{
    AccountCheck, MplCoreAccount, OptionalAccountCheck, ProcessInstruction, SignerAccount,
    SystemAccount, ToOptionalAccount, WritableAccount,
};

#[derive(Debug)]
pub struct UpdateNftV1Accounts<'a, 'info> {
    pub asset: &'a AccountInfo<'info>,
    pub collection: Option<&'a AccountInfo<'info>>,
    pub authority: Option<&'a AccountInfo<'info>>,
    pub payer: &'a AccountInfo<'info>,
    pub system_program: &'a AccountInfo<'info>,
    pub log_wrapper: Option<&'a AccountInfo<'info>>,
    pub mpl_core: &'a AccountInfo<'info>,
}

impl<'a, 'info> TryFrom<&'a [AccountInfo<'info>]> for UpdateNftV1Accounts<'a, 'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo<'info>]) -> Result<Self, Self::Error> {
        let [asset, collection, authority, payer, system_program, log_wrapper, mpl_core] = accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        WritableAccount::check(asset)?;
        WritableAccount::check(collection)?;
        SignerAccount::check_optional(authority.to_optional())?;
        WritableAccount::check(payer)?;
        SignerAccount::check(payer)?;
        SystemAccount::check(system_program)?;
        MplCoreAccount::check(mpl_core)?;

        Ok(Self {
            asset,
            collection: collection.to_optional(),
            authority: authority.to_optional(),
            payer,
            system_program,
            log_wrapper: log_wrapper.to_optional(),
            mpl_core,
        })
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct UpdateNftV1InstructionData {
    pub new_name: Option<String>,
    pub new_uri: Option<String>,
}

pub struct UpdateNftV1<'a, 'info> {
    pub accounts: UpdateNftV1Accounts<'a, 'info>,
    pub instruction_data: UpdateNftV1InstructionData,
}

impl<'a, 'info> TryFrom<(&'a [AccountInfo<'info>], UpdateNftV1InstructionData)>
    for UpdateNftV1<'a, 'info>
{
    type Error = ProgramError;

    fn try_from(
        (accounts, instruction_data): (&'a [AccountInfo<'info>], UpdateNftV1InstructionData),
    ) -> Result<Self, Self::Error> {
        let accounts = UpdateNftV1Accounts::try_from(accounts)?;

        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a, 'info> ProcessInstruction for UpdateNftV1<'a, 'info> {
    fn process(self) -> ProgramResult {
        let mut update_cpi = UpdateV1CpiBuilder::new(self.accounts.mpl_core);

        update_cpi
            .asset(self.accounts.asset)
            .collection(self.accounts.collection)
            .authority(self.accounts.authority)
            .payer(self.accounts.payer)
            .system_program(self.accounts.system_program)
            .log_wrapper(self.accounts.log_wrapper);

        if let Some(name) = self.instruction_data.new_name {
            update_cpi.new_name(name);
        }

        if let Some(uri) = self.instruction_data.new_uri {
            update_cpi.new_uri(uri);
        }

        update_cpi.invoke()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::test_utils::*;
    use solana_program::pubkey::Pubkey;
    use solana_sdk_ids::system_program;

    #[test]
    fn test_create_nft_account_success() {
        let asset = new_test_account(Pubkey::new_unique(), false, true, 1, 0, system_program::ID);
        let collection =
            new_test_account(Pubkey::new_unique(), false, true, 1, 0, system_program::ID);
        let authority =
            new_test_account(Pubkey::new_unique(), true, false, 1, 0, system_program::ID);
        let payer = new_test_account(Pubkey::new_unique(), true, true, 1, 0, system_program::ID);
        let system_program =
            new_test_account(system_program::ID, false, false, 1, 0, system_program::ID);
        let log_wrapper =
            new_test_account(Pubkey::new_unique(), false, false, 1, 0, system_program::ID);
        let mpl_core = new_test_account(mpl_core::ID, false, false, 1, 0, mpl_core::ID);

        let accounts = vec![
            asset,
            collection,
            authority,
            payer,
            system_program,
            log_wrapper,
            mpl_core,
        ];

        let res = UpdateNftV1Accounts::try_from(accounts.as_slice());
        assert!(res.is_ok(), "expected Ok, but got Err: {:?}", res);
    }

    #[test]
    fn test_create_nft_account_wrong_system_program() {
        let asset = new_test_account(Pubkey::new_unique(), false, true, 1, 0, system_program::ID);
        let collection =
            new_test_account(Pubkey::new_unique(), false, true, 1, 0, system_program::ID);
        let authority =
            new_test_account(Pubkey::new_unique(), true, false, 1, 0, system_program::ID);
        let payer = new_test_account(Pubkey::new_unique(), true, true, 1, 0, system_program::ID);
        let bad_system_program = new_test_account(
            Pubkey::new_unique(),
            false,
            false,
            1,
            0,
            Pubkey::new_unique(),
        );
        let log_wrapper =
            new_test_account(Pubkey::new_unique(), false, false, 1, 0, system_program::ID);
        let mpl_core = new_test_account(mpl_core::ID, false, false, 1, 0, mpl_core::ID);

        let accounts = vec![
            asset,
            collection,
            authority,
            payer,
            bad_system_program,
            log_wrapper,
            mpl_core,
        ];

        let res = UpdateNftV1Accounts::try_from(accounts.as_slice());
        assert!(
            res.is_err(),
            "expected failure because system_program was wrong, but got Ok: {:?}",
            res,
        );
    }

    #[test]
    fn test_create_nft_account_wrong_mpl_core() {
        let asset = new_test_account(Pubkey::new_unique(), false, true, 1, 0, system_program::ID);
        let collection =
            new_test_account(Pubkey::new_unique(), false, true, 1, 0, system_program::ID);
        let authority =
            new_test_account(Pubkey::new_unique(), true, false, 1, 0, system_program::ID);
        let payer = new_test_account(Pubkey::new_unique(), true, true, 1, 0, system_program::ID);
        let system_program =
            new_test_account(system_program::ID, false, false, 1, 0, system_program::ID);
        let log_wrapper =
            new_test_account(Pubkey::new_unique(), false, false, 1, 0, system_program::ID);
        let bad_mpl_core = new_test_account(
            Pubkey::new_unique(),
            false,
            false,
            1,
            0,
            Pubkey::new_unique(),
        );

        let accounts = vec![
            asset,
            collection,
            authority,
            payer,
            system_program,
            log_wrapper,
            bad_mpl_core,
        ];

        let res = UpdateNftV1Accounts::try_from(accounts.as_slice());
        assert!(
            res.is_err(),
            "expected failure because mpl_core was wrong, but got Ok: {:?}",
            res
        );
    }

    #[test]
    fn test_create_nft_account_not_enough_accounts() {
        let accounts = vec![];
        let res = UpdateNftV1Accounts::try_from(accounts.as_slice());
        assert!(
            res.is_err(),
            "expected failure because account is not enough, but got Ok: {:?}",
            res
        );
    }
}

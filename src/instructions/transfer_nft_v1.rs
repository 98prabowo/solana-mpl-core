use mpl_core::instructions::TransferV1CpiBuilder;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::utils::{
    AccountCheck, MplCoreAccount, OptionalAccountCheck, ProcessInstruction, SignerAccount,
    SystemAccount, ToOptionalAccount, WritableAccount,
};

#[derive(Debug)]
pub struct TransferNftV1Accounts<'a, 'info> {
    pub asset: &'a AccountInfo<'info>,
    pub collection: Option<&'a AccountInfo<'info>>,
    pub authority: Option<&'a AccountInfo<'info>>,
    pub new_owner: &'a AccountInfo<'info>,
    pub payer: &'a AccountInfo<'info>,
    pub system_program: Option<&'a AccountInfo<'info>>,
    pub log_wrapper: Option<&'a AccountInfo<'info>>,
    pub mpl_core: &'a AccountInfo<'info>,
}

impl<'a, 'info> TryFrom<&'a [AccountInfo<'info>]> for TransferNftV1Accounts<'a, 'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo<'info>]) -> Result<Self, Self::Error> {
        let [asset, collection, authority, new_owner, payer, system_program, log_wrapper, mpl_core] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        WritableAccount::check(asset)?;
        WritableAccount::check_optional(collection.to_optional())?;
        SignerAccount::check_optional(authority.to_optional())?;
        WritableAccount::check(payer)?;
        SignerAccount::check(payer)?;
        SystemAccount::check_optional(system_program.to_optional())?;
        MplCoreAccount::check(mpl_core)?;

        Ok(Self {
            asset,
            collection: collection.to_optional(),
            authority: authority.to_optional(),
            new_owner,
            payer,
            system_program: system_program.to_optional(),
            log_wrapper: log_wrapper.to_optional(),
            mpl_core,
        })
    }
}

pub struct TransferNftV1<'a, 'info> {
    accounts: TransferNftV1Accounts<'a, 'info>,
}

impl<'a, 'info> TryFrom<&'a [AccountInfo<'info>]> for TransferNftV1<'a, 'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo<'info>]) -> Result<Self, Self::Error> {
        let accounts = TransferNftV1Accounts::try_from(accounts)?;

        Ok(Self { accounts })
    }
}

impl<'a, 'info> ProcessInstruction for TransferNftV1<'a, 'info> {
    fn process(self) -> ProgramResult {
        TransferV1CpiBuilder::new(self.accounts.mpl_core)
            .asset(self.accounts.asset)
            .collection(self.accounts.collection)
            .authority(self.accounts.authority)
            .new_owner(self.accounts.new_owner)
            .payer(self.accounts.payer)
            .system_program(self.accounts.system_program)
            .log_wrapper(self.accounts.log_wrapper)
            .invoke()?;

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
    fn test_transfer_nft_account_success() {
        let asset = new_test_account(Pubkey::new_unique(), false, true, 1, 0, system_program::ID);
        let collection =
            new_test_account(Pubkey::new_unique(), false, true, 1, 0, system_program::ID);
        let authority =
            new_test_account(Pubkey::new_unique(), true, false, 1, 0, system_program::ID);
        let new_owner =
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
            new_owner,
            payer,
            system_program,
            log_wrapper,
            mpl_core,
        ];

        let res = TransferNftV1Accounts::try_from(accounts.as_slice());
        assert!(res.is_ok(), "expected Ok, but got Err: {:?}", res);
    }

    #[test]
    fn test_transfer_nft_account_wrong_system_program() {
        let asset = new_test_account(Pubkey::new_unique(), false, true, 1, 0, system_program::ID);
        let collection =
            new_test_account(Pubkey::new_unique(), false, true, 1, 0, system_program::ID);
        let authority =
            new_test_account(Pubkey::new_unique(), true, false, 1, 0, system_program::ID);
        let new_owner =
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
            new_owner,
            payer,
            bad_system_program,
            log_wrapper,
            mpl_core,
        ];

        let res = TransferNftV1Accounts::try_from(accounts.as_slice());
        assert!(
            res.is_err(),
            "expected failure because system_program was wrong, but got Ok: {:?}",
            res,
        );
    }

    #[test]
    fn test_transfer_nft_account_wrong_mpl_core() {
        let asset = new_test_account(Pubkey::new_unique(), false, true, 1, 0, system_program::ID);
        let collection =
            new_test_account(Pubkey::new_unique(), false, true, 1, 0, system_program::ID);
        let authority =
            new_test_account(Pubkey::new_unique(), true, false, 1, 0, system_program::ID);
        let new_owner =
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
            new_owner,
            payer,
            system_program,
            log_wrapper,
            bad_mpl_core,
        ];

        let res = TransferNftV1Accounts::try_from(accounts.as_slice());
        assert!(
            res.is_err(),
            "expected failure because mpl_core was wrong, but got Ok: {:?}",
            res
        );
    }

    #[test]
    fn test_transfer_nft_account_not_enough_accounts() {
        let accounts = vec![];
        let res = TransferNftV1Accounts::try_from(accounts.as_slice());
        assert!(
            res.is_err(),
            "expected failure because account is not enough, but got Ok: {:?}",
            res
        );
    }
}

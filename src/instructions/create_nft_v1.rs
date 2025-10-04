use borsh::{BorshDeserialize, BorshSerialize};
use mpl_core::{
    instructions::CreateV1CpiBuilder,
    types::{DataState, PluginAuthorityPair},
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
};

use crate::utils::{
    AccountCheck, MplCoreAccount, OptionalAccountCheck, ProcessInstruction, SignerAccount,
    SystemAccount, ToOptionalAccount, WritableAccount,
};

#[derive(Debug)]
pub struct CreateNftV1Accounts<'a, 'info> {
    pub asset: &'a AccountInfo<'info>,
    pub collection: Option<&'a AccountInfo<'info>>,
    pub authority: Option<&'a AccountInfo<'info>>,
    pub payer: &'a AccountInfo<'info>,
    pub owner: Option<&'a AccountInfo<'info>>,
    pub update_authority: Option<&'a AccountInfo<'info>>,
    pub system_program: &'a AccountInfo<'info>,
    pub log_wrapper: Option<&'a AccountInfo<'info>>,
    pub mpl_core: &'a AccountInfo<'info>,
}

impl<'a, 'info> TryFrom<&'a [AccountInfo<'info>]> for CreateNftV1Accounts<'a, 'info> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo<'info>]) -> Result<Self, Self::Error> {
        let [
            asset,
            collection,
            authority,
            payer,
            owner,
            update_authority,
            system_program,
            log_wrapper,
            mpl_core,
        ] = accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        WritableAccount::check(asset)?;
        WritableAccount::check(collection)?;
        SignerAccount::check_optional(authority.to_optional())?;
        WritableAccount::check(payer)?;
        SignerAccount::check(payer)?;
        SignerAccount::check_optional(owner.to_optional())?;
        SignerAccount::check_optional(update_authority.to_optional())?;
        SystemAccount::check(system_program)?;
        MplCoreAccount::check(mpl_core)?;

        Ok(Self {
            asset,
            collection: collection.to_optional(),
            authority: authority.to_optional(),
            payer,
            owner: owner.to_optional(),
            update_authority: update_authority.to_optional(),
            system_program,
            log_wrapper: log_wrapper.to_optional(),
            mpl_core,
        })
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct CreateNftV1InstructionData {
    pub data_state: Option<DataState>,
    pub name: String,
    pub uri: String,
    pub plugins: Option<Vec<PluginAuthorityPair>>,
}

#[derive(Debug)]
pub struct CreateNftV1<'a, 'info> {
    pub accounts: CreateNftV1Accounts<'a, 'info>,
    pub instruction_data: CreateNftV1InstructionData,
}

impl<'a, 'info> TryFrom<(&'a [AccountInfo<'info>], CreateNftV1InstructionData)>
    for CreateNftV1<'a, 'info>
{
    type Error = ProgramError;

    fn try_from(
        (accounts, instruction_data): (&'a [AccountInfo<'info>], CreateNftV1InstructionData),
    ) -> Result<Self, Self::Error> {
        let accounts = CreateNftV1Accounts::try_from(accounts)?;

        Ok(Self {
            accounts,
            instruction_data,
        })
    }
}

impl<'a, 'info> ProcessInstruction for CreateNftV1<'a, 'info> {
    fn process(self) -> ProgramResult {
        CreateV1CpiBuilder::new(self.accounts.mpl_core)
            .asset(self.accounts.asset)
            .collection(self.accounts.collection)
            .authority(self.accounts.authority)
            .payer(self.accounts.payer)
            .owner(self.accounts.owner)
            .update_authority(self.accounts.update_authority)
            .system_program(self.accounts.system_program)
            .data_state(
                self.instruction_data
                    .data_state
                    .unwrap_or(DataState::AccountState),
            )
            .log_wrapper(self.accounts.log_wrapper)
            .name(self.instruction_data.name)
            .uri(self.instruction_data.uri)
            .plugins(self.instruction_data.plugins.unwrap_or_default())
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
    fn test_create_nft_account_success() {
        let asset = new_test_account(Pubkey::new_unique(), false, true, 1, 0, system_program::ID);
        let collection =
            new_test_account(Pubkey::new_unique(), false, true, 1, 0, system_program::ID);
        let authority =
            new_test_account(Pubkey::new_unique(), true, false, 1, 0, system_program::ID);
        let payer = new_test_account(Pubkey::new_unique(), true, true, 1, 0, system_program::ID);
        let owner = new_test_account(Pubkey::new_unique(), true, false, 1, 0, system_program::ID);
        let update_authority =
            new_test_account(Pubkey::new_unique(), true, false, 1, 0, system_program::ID);
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
            owner,
            update_authority,
            system_program,
            log_wrapper,
            mpl_core,
        ];

        let res = CreateNftV1Accounts::try_from(accounts.as_slice());
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
        let owner = new_test_account(Pubkey::new_unique(), true, false, 1, 0, system_program::ID);
        let update_authority =
            new_test_account(Pubkey::new_unique(), true, false, 1, 0, system_program::ID);
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
            owner,
            update_authority,
            bad_system_program,
            log_wrapper,
            mpl_core,
        ];

        let res = CreateNftV1Accounts::try_from(accounts.as_slice());
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
        let owner = new_test_account(Pubkey::new_unique(), true, false, 1, 0, system_program::ID);
        let update_authority =
            new_test_account(Pubkey::new_unique(), true, false, 1, 0, system_program::ID);
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
            owner,
            update_authority,
            system_program,
            log_wrapper,
            bad_mpl_core,
        ];

        let res = CreateNftV1Accounts::try_from(accounts.as_slice());
        assert!(
            res.is_err(),
            "expected failure because mpl_core was wrong, but got Ok: {:?}",
            res
        );
    }

    #[test]
    fn test_create_nft_account_not_enough_accounts() {
        let accounts = vec![];
        let res = CreateNftV1Accounts::try_from(accounts.as_slice());
        assert!(
            res.is_err(),
            "expected failure because account is not enough, but got Ok: {:?}",
            res
        );
    }
}

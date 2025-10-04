use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::{
    instructions::{CreateNftV1, Instructions, TransferNftV1, UpdateNftV1},
    utils::ProcessInstruction,
};

pub fn process_entrypoint(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instructions::try_from_slice(instruction_data)?;

    match instruction {
        Instructions::CreateNftV1(data) => CreateNftV1::try_from((accounts, data))?.process(),
        Instructions::UpdateNftV1(data) => UpdateNftV1::try_from((accounts, data))?.process(),
        Instructions::TransferNftV1 => TransferNftV1::try_from(accounts)?.process(),
    }
}

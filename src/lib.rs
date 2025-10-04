mod instructions;
mod processor;
mod utils;

use solana_program::{
    account_info::AccountInfo, declare_id, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

declare_id!("3KRCmsnNYQvjp1TZaha1riRmx5GGVt67yv2sxDsFXbRG");

entrypoint!(process_entrypoint);

fn process_entrypoint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    processor::process_entrypoint(program_id, accounts, instruction_data)
}

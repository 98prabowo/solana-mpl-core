pub mod create_nft_v1;
pub mod transfer_nft_v1;
pub mod update_nft_v1;

pub use create_nft_v1::*;
pub use transfer_nft_v1::*;
pub use update_nft_v1::*;

use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum Instructions {
    CreateNftV1(CreateNftV1InstructionData),
    UpdateNftV1(UpdateNftV1InstructionData),
    TransferNftV1,
}

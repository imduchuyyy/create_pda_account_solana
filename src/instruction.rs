use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::Error::InvalidInstruction;

pub enum CreatePDAInstruction {
    CreatePDA {
        lamports: u64,
        space: u64
    }
}

impl CreatePDAInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => {
                let (lamports, rest) = Self::unpack_u64(rest)?;
                let (space, _) = Self::unpack_u64(rest)?;
                Self::CreatePDA{
                    lamports,
                    space,
                }
            }
            _ => return Err(InvalidInstruction.into()),
        })
    }

    pub fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (amount, rest) = input.split_at(8);
            let amount = amount.get(..8).and_then(|slice| slice.try_into().ok()).map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
            Ok((amount, rest))
        } else {
            Err(InvalidInstruction.into())
        }
    }
}

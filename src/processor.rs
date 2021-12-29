use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke},
    pubkey::Pubkey,
    system_instruction
};

use crate::{instruction::CreatePDAInstruction};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8]
    ) -> ProgramResult {
        let instruction = CreatePDAInstruction::unpack(instruction_data)?;

        match instruction {
            CreatePDAInstruction::CreatePDA { lamports, space } => {
                msg!("Coin98Instruction: CreatePDA");
                Self::process_create_pda(accounts, lamports, space, program_id)
            },
        }
    }

    pub fn process_create_pda(
        accounts: &[AccountInfo],
        lamports: u64,
        space: u64,
        program_id: &Pubkey
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let creator = next_account_info(account_info_iter)?;
        let new_account = next_account_info(account_info_iter)?;

        let create_account_instruction = system_instruction::create_account(
            creator.key,
            new_account.key,
            lamports,
            space,
            program_id
        );

        invoke(
            &create_account_instruction,
            &[
                creator.clone(),
                new_account.clone()
            ],
        )?;

        Ok(())
    }
}


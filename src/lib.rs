use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};
use borsh::{BorshDeserialize, BorshSerialize};


solana_program::declare_id!("solatrack11111111111111111111111111111111");


#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Product {
    pub product_id: String,     
    pub name: String,           
    pub manufacturer: String,   
    pub manufacturing_date: i64, 
    pub current_location: String,
    pub temperature: f32,        
    pub humidity: f32,          
    pub last_updated: i64,       
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum SolaTrackInstruction {
    InitProduct {
        product_id: String,
        name: String,
        manufacturer: String,
    },
    UpdateProductData {
        location: String,
        temperature: f32,
        humidity: f32,
    },
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = SolaTrackInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        SolaTrackInstruction::InitProduct { product_id, name, manufacturer } => {
            msg!("Instruction: InitProduct");
            process_init_product(program_id, accounts, product_id, name, manufacturer)
        },
        SolaTrackInstruction::UpdateProductData { location, temperature, humidity } => {
            msg!("Instruction: UpdateProductData");
            process_update_product(program_id, accounts, location, temperature, humidity)
        },
    }
}

fn process_init_product(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    product_id: String,
    name: String,
    manufacturer: String,
) -> ProgramResult {
    // Implementation details here
    msg!("Product initialized successfully");
    Ok(())
}

fn process_update_product(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    location: String,
    temperature: f32,
    humidity: f32,
) -> ProgramResult {
    // Implementation details here
    msg!("Product data updated successfully");
    Ok(())
}

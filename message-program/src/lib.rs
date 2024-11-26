use std::{num::NonZeroU64, u64};

use anyhow::{Ok, Result};
use bytemuck::{Pod, Zeroable};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, PartialEq, Eq)]
pub struct Messege {
    // u8 but padded to impl pod
    pub discriminator: u64,
    // sender pubkey
    pub sender_id: [u8; 32],
    // size of the Messege
    pub size: u64,
    // validator priority fees
    pub priority_fee: Option<NonZeroU64>,
    // content of the messege
    pub data: [u8; 1024],
}
impl Messege {
    pub const LEN: usize = size_of::<Self>();
}

#[derive(TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum Instructions {
    Initialize = 3,
    Close = 6,
    Update = 9,
}

pub fn send_instruction(data: &[u8]) -> Result<()> {
    let (discriminator, data) = data.split_first().unwrap();
    process_instruction(*discriminator, data)?;
    Ok(())
}

fn process_instruction(discriminator: u8, data: &[u8]) -> Result<()> {
    match Instructions::try_from(discriminator)? {
        Instructions::Initialize => initialize_messege(data),
        Instructions::Close => close_messege(data),
        Instructions::Update => update_messege(data),
    }
}

fn initialize_messege(data: &[u8]) -> Result<()> {
    let msg = bytemuck::from_bytes::<Messege>(data);
    println!("Allocatin {:?} bytes for {:?}...", msg.size, msg.sender_id);
    Ok(())
}
fn close_messege(data: &[u8]) -> Result<()> {
    let msg = bytemuck::try_from_bytes::<Messege>(data).unwrap();
    assert!(msg.priority_fee.is_some());
    println!("{:?} deleted.", msg.size);
    Ok(())
}
fn update_messege(data: &[u8]) -> Result<()> {
    let msg = bytemuck::try_from_bytes::<Messege>(data).unwrap();
    println!(
        "collected {:?} lamports from {:?}",
        msg.priority_fee.unwrap(),
        msg.sender_id
    );
    Ok(())
}

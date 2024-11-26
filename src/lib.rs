use std::num::NonZeroU64;

use anyhow::{Ok, Result};
use bytemuck::{Pod, Zeroable};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[repr(C)]
#[derive(Pod, Zeroable, Clone, Copy, PartialEq, Eq)]
pub struct Messege {
    // u8 but padded to impl pod
    discriminator: u64,
    // sender pubkey
    sender_id: [u8; 32],
    // size of the Messege
    size: u64,
    // validator priority fees
    priority_fee: Option<NonZeroU64>,
    // content of the messege
    data: [u8; 1024],
}
impl Messege {
    pub const LEN: usize = size_of::<Self>();
}

#[derive(TryFromPrimitive, IntoPrimitive)]
#[repr(u8)]
enum Instructions {
    Initialize = 0,
    Close = 1,
    Update = 2,
}

pub fn send_instruction(ix: Messege) -> Result<()> {
    let (discriminator, data) = bytemuck::bytes_of::<Messege>(&ix).split_first().unwrap();
    process_instruction(*discriminator, data)?;
    Ok(())
}

pub fn process_instruction(discriminator: u8, data: &[u8]) -> Result<()> {
    match Instructions::try_from(discriminator)? {
        Instructions::Initialize => initialize_messege(data),
        Instructions::Close => close_messege(data),
        Instructions::Update => update_messege(data),
    }
}

pub fn initialize_messege(data: &[u8]) -> Result<()> {
    let msg = bytemuck::from_bytes::<Messege>(data);
    println!("Allocatin {:?} bytes for {:?}...", msg.size, msg.sender_id);
    Ok(())
}
pub fn close_messege(data: &[u8]) -> Result<()> {
    let msg = bytemuck::from_bytes::<Messege>(data);
    println!("{:?} deleted.", msg.size);
    Ok(())
}
pub fn update_messege(data: &[u8]) -> Result<()> {
    let msg = bytemuck::from_bytes::<Messege>(data);
    println!(
        "collected {:?} lamports from {:?}",
        msg.priority_fee.unwrap(),
        msg.sender_id
    );
    Ok(())
}

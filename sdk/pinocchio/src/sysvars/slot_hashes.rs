use super::Sysvar;
use crate::{program_error::ProgramError, pubkey::Pubkey};

/// The ID of the slot hashes sysvar.
pub const SLOTHASHES_ID: Pubkey = [
    6, 167, 213, 23, 25, 47, 10, 175, 198, 242, 101, 227, 251, 119, 204, 122, 218, 130, 197, 41,
    208, 190, 59, 19, 110, 45, 0, 85, 32, 0, 0, 0,
];

pub struct SlotHashes {
    raw: *const u8,
}

pub struct SlotHash {
    height: u64,
    hash: [u8; 32],
}

impl SlotHashes {
    pub fn get_slothashes_len(&self) -> u64 {
        unsafe { u64::from_le(*(self.raw as *const u64)) }
    }

    pub unsafe fn _get_slot_hash(&self, index: u64) -> SlotHash {
        *(self.raw.add(8 + index as usize * 40) as SlotHash)
    }

    pub fn get_slot_hash(&self, index: u64) -> Result<SlotHash, ProgramError> {
        if index > self.get_slothashes_len() {
            return Err(ProgramError::InvalidAccountData);
        }
        unsafe { Ok(self._get_slot_hash(index)) }
    }
}

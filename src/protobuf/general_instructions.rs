// Automatically generated rust module for 'general_instructions.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct GeneralInstructions {
    pub main_instructions: mod_GeneralInstructions::MainInstrEnum,
}

impl<'a> MessageRead<'a> for GeneralInstructions {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.main_instructions = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for GeneralInstructions {
    fn get_size(&self) -> usize {
        0
        + if self.main_instructions == general_instructions::mod_GeneralInstructions::MainInstrEnum::DO_NOTHING { 0 } else { 1 + sizeof_varint(*(&self.main_instructions) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.main_instructions != general_instructions::mod_GeneralInstructions::MainInstrEnum::DO_NOTHING { w.write_with_tag(8, |w| w.write_enum(*&self.main_instructions as i32))?; }
        Ok(())
    }
}

pub mod mod_GeneralInstructions {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MainInstrEnum {
    DO_NOTHING = 0,
    REBOOT = 1,
    FREE_MEM = 2,
    FLASH_LED = 3,
    FLASH_GREEN = 4,
    FLASH_BLUE = 5,
}

impl Default for MainInstrEnum {
    fn default() -> Self {
        MainInstrEnum::DO_NOTHING
    }
}

impl From<i32> for MainInstrEnum {
    fn from(i: i32) -> Self {
        match i {
            0 => MainInstrEnum::DO_NOTHING,
            1 => MainInstrEnum::REBOOT,
            2 => MainInstrEnum::FREE_MEM,
            3 => MainInstrEnum::FLASH_LED,
            4 => MainInstrEnum::FLASH_GREEN,
            5 => MainInstrEnum::FLASH_BLUE,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for MainInstrEnum {
    fn from(s: &'a str) -> Self {
        match s {
            "DO_NOTHING" => MainInstrEnum::DO_NOTHING,
            "REBOOT" => MainInstrEnum::REBOOT,
            "FREE_MEM" => MainInstrEnum::FREE_MEM,
            "FLASH_LED" => MainInstrEnum::FLASH_LED,
            "FLASH_GREEN" => MainInstrEnum::FLASH_GREEN,
            "FLASH_BLUE" => MainInstrEnum::FLASH_BLUE,
            _ => Self::default(),
        }
    }
}

}


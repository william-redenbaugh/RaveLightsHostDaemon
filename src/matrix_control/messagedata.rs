// Automatically generated rust module for 'messagedata.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]

extern crate quick_protobuf; 

use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MessageData {
    pub message_size: u32,
    pub message_type: mod_MessageData::MessageType,
    pub return_message: bool,
}

impl<'a> MessageRead<'a> for MessageData {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.message_size = r.read_uint32(bytes)?,
                Ok(16) => msg.message_type = r.read_enum(bytes)?,
                Ok(24) => msg.return_message = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for MessageData {
    fn get_size(&self) -> usize {
        0
        + if self.message_size == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.message_size) as u64) }
        + if self.message_type == messagedata::mod_MessageData::MessageType::GENERAL_INSTRUCTIONS { 0 } else { 1 + sizeof_varint(*(&self.message_type) as u64) }
        + if self.return_message == false { 0 } else { 1 + sizeof_varint(*(&self.return_message) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.message_size != 0u32 { w.write_with_tag(8, |w| w.write_uint32(*&self.message_size))?; }
        if self.message_type != messagedata::mod_MessageData::MessageType::GENERAL_INSTRUCTIONS { w.write_with_tag(16, |w| w.write_enum(*&self.message_type as i32))?; }
        if self.return_message != false { w.write_with_tag(24, |w| w.write_bool(*&self.return_message))?; }
        Ok(())
    }
}

pub mod mod_MessageData {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageType {
    GENERAL_INSTRUCTIONS = 0,
    MATRIX_DATA = 1,
    LED_STRIP_DATA = 2,
    HEAAT_CONTROL_DATA = 3,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::GENERAL_INSTRUCTIONS
    }
}

impl From<i32> for MessageType {
    fn from(i: i32) -> Self {
        match i {
            0 => MessageType::GENERAL_INSTRUCTIONS,
            1 => MessageType::MATRIX_DATA,
            2 => MessageType::LED_STRIP_DATA,
            3 => MessageType::HEAAT_CONTROL_DATA,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for MessageType {
    fn from(s: &'a str) -> Self {
        match s {
            "GENERAL_INSTRUCTIONS" => MessageType::GENERAL_INSTRUCTIONS,
            "MATRIX_DATA" => MessageType::MATRIX_DATA,
            "LED_STRIP_DATA" => MessageType::LED_STRIP_DATA,
            "HEAAT_CONTROL_DATA" => MessageType::HEAAT_CONTROL_DATA,
            _ => Self::default(),
        }
    }
}

}


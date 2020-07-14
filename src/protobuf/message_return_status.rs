// Automatically generated rust module for 'message_return_status.proto' file

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
pub struct ReturnMessage {
    pub message_status: mod_ReturnMessage::MessageStatus,
}

impl<'a> MessageRead<'a> for ReturnMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.message_status = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ReturnMessage {
    fn get_size(&self) -> usize {
        0
        + if self.message_status == message_return_status::mod_ReturnMessage::MessageStatus::CMD_ACCEPTED { 0 } else { 1 + sizeof_varint(*(&self.message_status) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.message_status != message_return_status::mod_ReturnMessage::MessageStatus::CMD_ACCEPTED { w.write_with_tag(8, |w| w.write_enum(*&self.message_status as i32))?; }
        Ok(())
    }
}

pub mod mod_ReturnMessage {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageStatus {
    CMD_ACCEPTED = 0,
    CMD_FAILED = 1,
    CMD_UNRECOGNIZED = 2,
    CMD_NOT_ON_DEVICE = 3,
    NO_CMD = 4,
}

impl Default for MessageStatus {
    fn default() -> Self {
        MessageStatus::CMD_ACCEPTED
    }
}

impl From<i32> for MessageStatus {
    fn from(i: i32) -> Self {
        match i {
            0 => MessageStatus::CMD_ACCEPTED,
            1 => MessageStatus::CMD_FAILED,
            2 => MessageStatus::CMD_UNRECOGNIZED,
            3 => MessageStatus::CMD_NOT_ON_DEVICE,
            4 => MessageStatus::NO_CMD,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for MessageStatus {
    fn from(s: &'a str) -> Self {
        match s {
            "CMD_ACCEPTED" => MessageStatus::CMD_ACCEPTED,
            "CMD_FAILED" => MessageStatus::CMD_FAILED,
            "CMD_UNRECOGNIZED" => MessageStatus::CMD_UNRECOGNIZED,
            "CMD_NOT_ON_DEVICE" => MessageStatus::CMD_NOT_ON_DEVICE,
            "NO_CMD" => MessageStatus::NO_CMD,
            _ => Self::default(),
        }
    }
}

}


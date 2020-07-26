// Automatically generated rust module for 'heart_program_control.proto' file

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
pub struct HeartControlData {
    pub en_sleep_mode: bool,
    pub en_lamp: bool,
    pub en_lock_mode: bool,
}

impl<'a> MessageRead<'a> for HeartControlData {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.en_sleep_mode = r.read_bool(bytes)?,
                Ok(16) => msg.en_lamp = r.read_bool(bytes)?,
                Ok(24) => msg.en_lock_mode = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for HeartControlData {
    fn get_size(&self) -> usize {
        0
        + if self.en_sleep_mode == false { 0 } else { 1 + sizeof_varint(*(&self.en_sleep_mode) as u64) }
        + if self.en_lamp == false { 0 } else { 1 + sizeof_varint(*(&self.en_lamp) as u64) }
        + if self.en_lock_mode == false { 0 } else { 1 + sizeof_varint(*(&self.en_lock_mode) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.en_sleep_mode != false { w.write_with_tag(8, |w| w.write_bool(*&self.en_sleep_mode))?; }
        if self.en_lamp != false { w.write_with_tag(16, |w| w.write_bool(*&self.en_lamp))?; }
        if self.en_lock_mode != false { w.write_with_tag(24, |w| w.write_bool(*&self.en_lock_mode))?; }
        Ok(())
    }
}


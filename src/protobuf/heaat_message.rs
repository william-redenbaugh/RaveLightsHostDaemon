// Automatically generated rust module for 'heaat_message.proto' file

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
pub struct HeaatMessage {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
    pub brightness: i32,
}

impl<'a> MessageRead<'a> for HeaatMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.red = r.read_int32(bytes)?,
                Ok(16) => msg.green = r.read_int32(bytes)?,
                Ok(24) => msg.blue = r.read_int32(bytes)?,
                Ok(32) => msg.brightness = r.read_int32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for HeaatMessage {
    fn get_size(&self) -> usize {
        0
        + if self.red == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.red) as u64) }
        + if self.green == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.green) as u64) }
        + if self.blue == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.blue) as u64) }
        + if self.brightness == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.brightness) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.red != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.red))?; }
        if self.green != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.green))?; }
        if self.blue != 0i32 { w.write_with_tag(24, |w| w.write_int32(*&self.blue))?; }
        if self.brightness != 0i32 { w.write_with_tag(32, |w| w.write_int32(*&self.brightness))?; }
        Ok(())
    }
}


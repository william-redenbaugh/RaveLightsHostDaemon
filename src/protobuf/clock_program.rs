// Automatically generated rust module for 'clock_program.proto' file

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
pub struct ClockProgram {
    pub hour_offset: i32,
    pub fade_animation_del: i32,
    pub en_hourly_messages: bool,
    pub en_blink_heart: bool,
    pub en_display: bool,
}

impl<'a> MessageRead<'a> for ClockProgram {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.hour_offset = r.read_int32(bytes)?,
                Ok(16) => msg.fade_animation_del = r.read_int32(bytes)?,
                Ok(24) => msg.en_hourly_messages = r.read_bool(bytes)?,
                Ok(32) => msg.en_blink_heart = r.read_bool(bytes)?,
                Ok(40) => msg.en_display = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for ClockProgram {
    fn get_size(&self) -> usize {
        0
        + if self.hour_offset == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.hour_offset) as u64) }
        + if self.fade_animation_del == 0i32 { 0 } else { 1 + sizeof_varint(*(&self.fade_animation_del) as u64) }
        + if self.en_hourly_messages == false { 0 } else { 1 + sizeof_varint(*(&self.en_hourly_messages) as u64) }
        + if self.en_blink_heart == false { 0 } else { 1 + sizeof_varint(*(&self.en_blink_heart) as u64) }
        + if self.en_display == false { 0 } else { 1 + sizeof_varint(*(&self.en_display) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.hour_offset != 0i32 { w.write_with_tag(8, |w| w.write_int32(*&self.hour_offset))?; }
        if self.fade_animation_del != 0i32 { w.write_with_tag(16, |w| w.write_int32(*&self.fade_animation_del))?; }
        if self.en_hourly_messages != false { w.write_with_tag(24, |w| w.write_bool(*&self.en_hourly_messages))?; }
        if self.en_blink_heart != false { w.write_with_tag(32, |w| w.write_bool(*&self.en_blink_heart))?; }
        if self.en_display != false { w.write_with_tag(40, |w| w.write_bool(*&self.en_display))?; }
        Ok(())
    }
}


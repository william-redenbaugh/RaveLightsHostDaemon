// Automatically generated rust module for 'temp_hum.proto' file

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
pub struct TempuratureHumidityMessage {
    pub tempurature_c: f32,
    pub humiditiy_relative: f32,
    pub valid_data: bool,
}

impl<'a> MessageRead<'a> for TempuratureHumidityMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(13) => msg.tempurature_c = r.read_float(bytes)?,
                Ok(21) => msg.humiditiy_relative = r.read_float(bytes)?,
                Ok(24) => msg.valid_data = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for TempuratureHumidityMessage {
    fn get_size(&self) -> usize {
        0
        + if self.tempurature_c == 0f32 { 0 } else { 1 + 4 }
        + if self.humiditiy_relative == 0f32 { 0 } else { 1 + 4 }
        + if self.valid_data == false { 0 } else { 1 + sizeof_varint(*(&self.valid_data) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.tempurature_c != 0f32 { w.write_with_tag(13, |w| w.write_float(*&self.tempurature_c))?; }
        if self.humiditiy_relative != 0f32 { w.write_with_tag(21, |w| w.write_float(*&self.humiditiy_relative))?; }
        if self.valid_data != false { w.write_with_tag(24, |w| w.write_bool(*&self.valid_data))?; }
        Ok(())
    }
}


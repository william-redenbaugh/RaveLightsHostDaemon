// Automatically generated rust module for 'led_strip_data.proto' file

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
pub struct LEDData {
    pub message_type: mod_LEDData::MessageType,
    pub kelvin_red_hue: u32,
    pub green_saturation: u32,
    pub blue_value: u32,
}

impl<'a> MessageRead<'a> for LEDData {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.message_type = r.read_enum(bytes)?,
                Ok(16) => msg.kelvin_red_hue = r.read_uint32(bytes)?,
                Ok(24) => msg.green_saturation = r.read_uint32(bytes)?,
                Ok(32) => msg.blue_value = r.read_uint32(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LEDData {
    fn get_size(&self) -> usize {
        0
        + if self.message_type == led_strip_data::mod_LEDData::MessageType::KELVIN_DATA { 0 } else { 1 + sizeof_varint(*(&self.message_type) as u64) }
        + if self.kelvin_red_hue == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.kelvin_red_hue) as u64) }
        + if self.green_saturation == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.green_saturation) as u64) }
        + if self.blue_value == 0u32 { 0 } else { 1 + sizeof_varint(*(&self.blue_value) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.message_type != led_strip_data::mod_LEDData::MessageType::KELVIN_DATA { w.write_with_tag(8, |w| w.write_enum(*&self.message_type as i32))?; }
        if self.kelvin_red_hue != 0u32 { w.write_with_tag(16, |w| w.write_uint32(*&self.kelvin_red_hue))?; }
        if self.green_saturation != 0u32 { w.write_with_tag(24, |w| w.write_uint32(*&self.green_saturation))?; }
        if self.blue_value != 0u32 { w.write_with_tag(32, |w| w.write_uint32(*&self.blue_value))?; }
        Ok(())
    }
}

pub mod mod_LEDData {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MessageType {
    KELVIN_DATA = 0,
    RGB_DATA = 1,
    HSV_DATA = 2,
}

impl Default for MessageType {
    fn default() -> Self {
        MessageType::KELVIN_DATA
    }
}

impl From<i32> for MessageType {
    fn from(i: i32) -> Self {
        match i {
            0 => MessageType::KELVIN_DATA,
            1 => MessageType::RGB_DATA,
            2 => MessageType::HSV_DATA,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for MessageType {
    fn from(s: &'a str) -> Self {
        match s {
            "KELVIN_DATA" => MessageType::KELVIN_DATA,
            "RGB_DATA" => MessageType::RGB_DATA,
            "HSV_DATA" => MessageType::HSV_DATA,
            _ => Self::default(),
        }
    }
}

}


// Automatically generated rust module for 'data_req.proto' file

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
pub struct RequestMessage {
    pub reqeust_type: mod_RequestMessage::RequestType,
    pub verbose: bool,
}

impl<'a> MessageRead<'a> for RequestMessage {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(0) => msg.reqeust_type = r.read_enum(bytes)?,
                Ok(8) => msg.verbose = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for RequestMessage {
    fn get_size(&self) -> usize {
        0
        + if self.reqeust_type == data_req::mod_RequestMessage::RequestType::TEMP_HUM_DATA { 0 } else { 1 + sizeof_varint(*(&self.reqeust_type) as u64) }
        + if self.verbose == false { 0 } else { 1 + sizeof_varint(*(&self.verbose) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.reqeust_type != data_req::mod_RequestMessage::RequestType::TEMP_HUM_DATA { w.write_with_tag(0, |w| w.write_enum(*&self.reqeust_type as i32))?; }
        if self.verbose != false { w.write_with_tag(8, |w| w.write_bool(*&self.verbose))?; }
        Ok(())
    }
}

pub mod mod_RequestMessage {


#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum RequestType {
    TEMP_HUM_DATA = 0,
}

impl Default for RequestType {
    fn default() -> Self {
        RequestType::TEMP_HUM_DATA
    }
}

impl From<i32> for RequestType {
    fn from(i: i32) -> Self {
        match i {
            0 => RequestType::TEMP_HUM_DATA,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for RequestType {
    fn from(s: &'a str) -> Self {
        match s {
            "TEMP_HUM_DATA" => RequestType::TEMP_HUM_DATA,
            _ => Self::default(),
        }
    }
}

}


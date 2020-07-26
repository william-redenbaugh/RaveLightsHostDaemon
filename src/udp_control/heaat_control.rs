// The protobuffer crage that we are using
extern crate quick_protobuf;

// Protobuffer writing module
use quick_protobuf::Writer;
// Standard UDP socket library
use std::net::UdpSocket;

// Crate that contains our message data.
use crate::heaat_message;
use crate::messagedata;

pub struct HeaatControl {
    pub socket: UdpSocket,
    pub address_port: String,
}

impl HeaatControl {
    // Function that lets us set our latest rgb value
    pub fn set(&mut self, _r: i32, _g: i32, _b: i32, _brightness: i32) {
        let heaat_msg = heaat_message::HeaatMessage {
            red: _r,
            green: _g,
            blue: _b,
            brightness: _brightness,
        };

        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer
            .write_message(&heaat_msg)
            .expect("Couldn't write heaat message properly");

        // Don't need first byte of our array
        out.remove(0);
        // Converts into box/ptr array
        let msg = out.into_boxed_slice();
        // Send the data to the heaat system
        self.socket
            .send_to(&msg, &self.address_port)
            .expect("Couldn't send the data :(");
    }
}

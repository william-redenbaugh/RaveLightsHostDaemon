// The protobuffer crage that we are using
extern crate quick_protobuf; 
extern crate time;

// Protobuffer writing module
use quick_protobuf::Writer;
// Standard UDP socket library
use std::net::UdpSocket;

// Crate that contains our message data. 
use crate::messagedata;
use crate::relay_msg;

pub struct RelayControl{
    pub socket:UdpSocket,  
    pub address_port: String
}

impl RelayControl{
    pub fn set(&mut self, _en: bool){
        let relay_msg = relay_msg::RelayMessage{
            en: _en
        };

        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer
            .write_message(&relay_msg)
            .expect("Couldn't write heaat message properly");
        
        out.remove(0);
        let msg = out.into_boxed_slice();
        self.socket.send_to(&msg, &self.address_port).expect("Could not send the data");
    }
}
// Protobuffer message create
extern crate quick_protobuf; 
use quick_protobuf::Writer; 

// Messagedata protobuffer message
use crate::messagedata; 

extern crate serial;
use std::io::prelude::*;
use serial::prelude::*;

pub struct TeensyControl{
    // Out Array For Dealing with Serial TTY Port Stuff
    pub out_arr: Box<[u8]>,
    // Length of array
    pub len: u32,
    // Serial port passover
    pub serial_port: serial::unix::TTYPort
}

// Setting up the Serial Strip stuff. 
impl TeensyControl{
    // Setup the serial interface for the strip control
    pub fn begin_strip(&mut self){
        // Provides messagedata fields. 
        let val = messagedata::MessageData{
            message_size: self.len * 3 as u32, 
            message_type: messagedata::mod_MessageData::MessageType::LED_STRIP_DATA, 
            return_message: false
        };

        // Scopes out the protobuff messaging so we save memory
        {
            let mut out = Vec::new();
            let mut writer = Writer::new(&mut out);
            writer
                .write_message(&val)
                .expect("Message couldn't write properly");
            
            // Generally speaking the first 
            // Byte indicates the size of the array. 
            // But we don't require it for our purposes. 
            out.remove(0);

            // Converts into a boxed pointer. 
            let msg_fill = out.into_boxed_slice();

            // Fills in the message data that will
            // Indiciate what type of message this is!
            for x in 0.. (msg_fill.len()){
                self.out_arr[x] = msg_fill[x];
            }
        }
        
        // Configure Serial settings. 
        const SETTINGS: serial::PortSettings = serial::PortSettings {
            baud_rate:    serial::Baud115200,
            char_size:    serial::Bits8,
            parity:       serial::ParityNone,
            stop_bits:    serial::Stop1,
            flow_control: serial::FlowNone,
        };
        let _result = self.serial_port.configure(&SETTINGS); 
        
        // Update strip
        self.update_strip();
    }

    pub fn set_led(&mut self, _led: u32, _r: u8, _g: u8, _b: u8 ){
        // Spot in our array. 
        let mut spot: usize = (_led * 3) as usize + 16;
        
        // Sets our out array spots
        self.out_arr[spot] = _r;
        spot = spot + 1; 
        self.out_arr[spot] = _g; 
        spot = spot + 1;
        self.out_arr[spot] = _b;
    }

    pub fn update_strip(&mut self){
        let _result = self.serial_port.write(&self.out_arr);
    }
}
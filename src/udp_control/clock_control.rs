// The protobuffer crage that we are using
extern crate quick_protobuf; 

// Protobuffer writing module
use quick_protobuf::Writer;

// Standard UDP socket library
use std::net::UdpSocket;

// Crate that contains our clock programming data. 
use crate::{
        messagedata, 
        clock_program
    };

// Object constructor udp control of the clock
pub struct ClockControl{
    pub socket:UdpSocket, 
    pub address_port: String
}

fn set_clock(hour_offset: i32, fade_animation_del: i32, en_hourly_messages: bool, en_blink_heart: bool, en_display: bool) -> Box <[u8]>{
    let mut out_arr = Vec::new();
    let mut clock_instr_out = Vec::new();
    // Adding our clock program instructions
    {
        let msg = clock_program::ClockProgram{
            hour_offset: hour_offset, 
            fade_animation_del: fade_animation_del, 
            en_hourly_messages: en_hourly_messages, 
            en_blink_heart: en_blink_heart, 
            en_display: en_display
        };
        let mut writer = Writer::new(&mut clock_instr_out);
        writer
            .write_message(&msg)
            .expect("Message could not write properly :(");
        clock_instr_out.remove(0);            
    }
    
    {
        // Provides messagedata fields. 
        let val = messagedata::MessageData{
            message_size: (clock_instr_out.len()) as u32, 
            message_type: messagedata::mod_MessageData::MessageType::CLOCK_DATA, 
            return_message: false
        };
        
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer
            .write_message(&val)
            .expect("Message couldn't write properly");

        // Generally speaking the first 
        // Byte indicates the size of the array. 
        // But we don't require it for our purposes. 
        out.remove(0);
        
        // Fills in the message data that will
        // Indiciate what type of message this is!
        for x in 0.. (out.len()){
            out_arr.push(out[x]);
        }
        for x in 0.. (16 - out.len()){
            out_arr.push(0);
        }
    }    

    // Fills in the message data that will
    // Indiciate what type of message this is!
    for x in 0.. (clock_instr_out.len()){
        out_arr.push(clock_instr_out[x]);
    }

    // Return the out array
    return out_arr.into_boxed_slice();
}

fn en_clock(en: bool) -> Box<[u8]>{
    return set_clock(0, 10, false, en, en);
}

// Implementation for the UDP control of the clock
impl ClockControl{
    /*Depricated functions that are only here until I update my clock with newer code */
    pub fn on_legacy(&self){
        let on_cmd: [u8; 5] = [40, 40, 50, 65, 1]; 
        self.socket.send_to(&on_cmd, &self.address_port).expect("couldn't send data");
    }
    pub fn off_legacy(&self){
        let off_cmd: [u8; 5] = [40, 40, 50, 65, 0];
        self.socket.send_to(&off_cmd, &self.address_port).expect("couldn't send data");
    }

    pub fn on(&self){
        self.socket
                    .send_to(&en_clock(true), &self.address_port)
                    .expect("could not send our instruction data");
    }

    pub fn off(&self){
        self.socket
                    .send_to(&en_clock(false), &self.address_port)
                    .expect("could not send our instruction data");
    }
}
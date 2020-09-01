// Protobuffer message create
extern crate quick_protobuf; 
use quick_protobuf::Writer; 

// Serial crate
extern crate serial;
use std::io::prelude::*;
use serial::prelude::*;

// Internal Protobuffer Crate for
// our own messagess
use crate::messagedata; 
use crate::led_strip_data; 

// Refcel and Rc stuff so we can have multiple pointers to the same stuff
// Will be used for sharing a single serial object across multiple "objects", that will be found
// 
use std::rc::Rc;
use std::cell::{Cell, RefCell};

pub fn new_teensy_control(port_ref: Rc<RefCell<serial::unix::TTYPort>>) -> TeensyControl{
    let teensy = TeensyControl{
        serial_port: port_ref
    };

    return teensy;
}

pub fn generate_messagedata_header(message_size: u32, message_type: messagedata::mod_MessageData::MessageType, return_message: bool) -> Vec<u8>{
    // Generate the struct of mesage
    let val = messagedata::MessageData{
        message_size: message_size, 
        message_type: message_type, 
        return_message: return_message,  
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
    for x in 0..(16-out.len()){
        out.push(0);
    }
    return out; 
}

pub fn generate_main_hsv_packet(h: u8, s: u8, v: u8) -> Vec<u8>{
    let val = led_strip_data::LEDData{
        message_type: led_strip_data::mod_LEDData::MessageType::HSV_DATA,
        kelvin_red_hue: h as u32, 
        green_saturation: s as u32, 
        blue_value: v as u32
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

    return out; 
}

pub fn generate_main_rgb_packet(r: u8, g: u8, b: u8) -> Vec<u8>{
    let val = led_strip_data::LEDData{
        message_type: led_strip_data::mod_LEDData::MessageType::RGB_DATA,
        kelvin_red_hue: r as u32, 
        green_saturation: g as u32, 
        blue_value: b as u32
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

    return out; 
}

pub fn generate_main_kelvin_packet(kelvin: u32) -> Vec<u8>{
    let val = led_strip_data::LEDData{
        message_type: led_strip_data::mod_LEDData::MessageType::KELVIN_DATA,
        kelvin_red_hue: kelvin, 
        green_saturation: 0, 
        blue_value: 0
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

    return out; 
}

pub struct TeensyControl{
    // Serial port passover
    pub serial_port: Rc<RefCell<serial::unix::TTYPort>>
}

impl TeensyControl{
    pub fn set_main_rgb(&mut self, r: u8, g: u8, b: u8){
        let image_msg = generate_main_rgb_packet(r, g, b); 
        let mut message = generate_messagedata_header(
            image_msg.len() as u32, 
            messagedata::mod_MessageData::MessageType::LED_STRIP_DATA, 
            false
        ); 

        for x in image_msg{
            message.push(x); 
        }

        let _result = self.serial_port.borrow_mut().write(&message); 
    }
    
    pub fn set_main_hsv(&mut self, h: u8, s: u8, v: u8){
        let image_msg = generate_main_hsv_packet(h, s, v); 

        let mut message = generate_messagedata_header(
            image_msg.len() as u32, 
            messagedata::mod_MessageData::MessageType::LED_STRIP_DATA, 
            false
        ); 

        for x in image_msg{
            message.push(x);
        }
        
        let _result = self.serial_port.borrow_mut().write(&message); 
    }

    pub fn set_main_kelvin(&mut self, kelvin: u32){
        let image_msg = generate_main_kelvin_packet(kelvin); 

        let mut message = generate_messagedata_header(
            image_msg.len() as u32, 
            messagedata::mod_MessageData::MessageType::LED_STRIP_DATA, 
            false
        ); 

        for x in image_msg{
            message.push(x);
        }
        let _result = self.serial_port.borrow_mut().write(&message); 
    }
}
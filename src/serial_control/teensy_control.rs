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

pub struct TeensyControl{
    // Serial port passover
    pub serial_port: Rc<RefCell<serial::unix::TTYPort>>
}

impl TeensyControl{
    
}
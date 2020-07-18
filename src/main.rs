// Rust's Files
use std::net::UdpSocket;
use std::{thread, time};
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use std::rc::Rc;
use std::cell::{Cell, RefCell};

use std::io::prelude::*;
use serial::prelude::*;

// Our UDP Control files 
// That let us control the devices on our wifi network
mod udp_control; 
use udp_control::{heart_control, matrix_control, clock_control, heaat_control, relay_control};

// Serial control files. 
// Lets us control devices on the local serial interface
mod serial_control; 
use serial_control::{teensy_control};

// Serial module that we are using
extern crate serial;

// Protobuffer Messages
mod protobuf;
use protobuf::{messagedata, heaat_message, general_instructions, relay_msg};

extern crate yahoo_finance; 
use yahoo_finance::history;

fn main() {
    // Setup control with teensy over Serial port. 
    let mut p = serial::open("/dev/ttyAMA0").unwrap();
    let port_ref = Rc::new(RefCell::new(p));
    
    // Teensy Controller object
    let teensy = teensy_control::new_teensy_control(Rc::clone(&port_ref));

    // Create our "teensy strip controller"
    let teensy_strip = teensy_control::new_serial_strip(288, Rc::clone(&port_ref));
}
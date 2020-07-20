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
use crate::udp_control; 

// Serial control files. 
// Lets us control devices on the local serial interface
use crate::serial_control; 

// Serial module that we are using
extern crate serial;

// Internal Messaging to our matrix
pub struct MatrixMessagePacket{
    pub debug: bool
}
// Message return packet so we know how things went 
pub struct MatrixReturnPacket{
    pub success: bool
}

// Primary matrix thread that will deal with issuing messages to the matrix. 
pub fn matrix_main(rx: mpsc::Receiver<MatrixMessagePacket>, tx: mpsc::Sender<MatrixReturnPacket>){
    loop{
        let msg_status = rx.recv().unwrap();
        if(msg_status.debug){
            println!("Recieved debug message on matrix thread!");
        }
    }
}

// Teensy message that instructs the thread what to do. 
pub struct TeensyMessagePacket{
    pub debug: bool
}
// Teensy message return that tells the return thread the status of their request. 
pub struct TeensyReturnPacket{
    pub debug: bool
}
// Function that will deal with our primary teensy control code. 
pub fn teensy_main(rx: mpsc::Receiver<TeensyMessagePacket>, tx: mpsc::Sender<TeensyReturnPacket>){
    // Setup control with teensy over Serial port. 
    //let mut p = serial::open("/dev/ttyAMA0").unwrap();
    //let port_ref = Rc::new(RefCell::new(p));
    
    // Teensy Controller object
    //let teensy = serial_control::teensy_control::new_teensy_control(Rc::clone(&port_ref));
    // Create our strip controller. 
    //let teensy_strip = serial_control::strip_control::new_serial_strip(288, Rc::clone(&port_ref));
    loop{
        let msg_status = rx.recv().unwrap();
        if(msg_status.debug){
            println!("Received debug message on teensy thread!");
        }
    }
}

// Packet information for dealing with out relays
pub struct RelayMessagePacket{
    relay_en: bool
}
// Getting back the status when we issue a Relay return request. 
pub struct RelayReturnPacket{
    success: bool
}

// Example code of how we will implement threads to deal with our relay code
pub fn relay_main(rx: mpsc::Receiver<RelayMessagePacket>, tx: mpsc::Sender<RelayReturnPacket>){
    loop{
        thread::sleep(Duration::from_millis(1000));
    }
}

// Function that will control all of our tempurate control code, since it's just the relay controlling the AC
// RN, we are implementing it half assed. 
pub fn temp_main(rx: mpsc::Receiver<RelayMessagePacket>, tx: mpsc::Sender<RelayReturnPacket>){
    loop{
        let relay_status = rx.recv().unwrap();
        if(relay_status.relay_en){
            // TODO
        }
    }
}
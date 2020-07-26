// Rust's Files
use std::cell::{Cell, RefCell};
use std::net::UdpSocket;
use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{thread, time};

use serial::prelude::*;
use std::io::prelude::*;

// Our UDP Control files
// That let us control the devices on our wifi network
use crate::udp_control;

// Serial control files.
// Lets us control devices on the local serial interface
use crate::serial_control;

// Serial module that we are using
extern crate serial;

// Internal Messaging to our matrix
pub struct MatrixMessagePacket {
    pub debug: bool,
}
// Message return packet so we know how things went
pub struct MatrixReturnPacket {
    pub success: bool,
}

// Primary matrix thread that will deal with issuing messages to the matrix.
pub fn matrix_main(rx: mpsc::Receiver<MatrixMessagePacket>, tx: mpsc::Sender<MatrixReturnPacket>) {
    loop {
        let msg_status = rx.recv().unwrap();
        if (msg_status.debug) {
            println!("Recieved debug message on matrix thread!");
        }
    }
}

// Teensy message that instructs the thread what to do.
pub struct TeensyMessagePacket {
    pub debug: bool,
}
// Teensy message return that tells the return thread the status of their request.
pub struct TeensyReturnPacket {
    pub debug: bool,
}
// Function that will deal with our primary teensy control code.
pub fn teensy_main(rx: mpsc::Receiver<TeensyMessagePacket>, tx: mpsc::Sender<TeensyReturnPacket>) {
    // Setup control with teensy over Serial port.
    //let mut p = serial::open("/dev/ttyAMA0").unwrap();
    //let port_ref = Rc::new(RefCell::new(p));

    // Teensy Controller object
    //let teensy = serial_control::teensy_control::new_teensy_control(Rc::clone(&port_ref));
    // Create our strip controller.
    //let teensy_strip = serial_control::strip_control::new_serial_strip(288, Rc::clone(&port_ref));
    loop {
        let msg_status = rx.recv().unwrap();
        if (msg_status.debug) {
            println!("Received debug message on teensy thread!");
        }
    }
}

// Packet information for dealing with out relays
pub struct RelayMessagePacket {
    pub relay_en: bool,
}
// Getting back the status when we issue a Relay return request.
pub struct RelayReturnPacket {
    pub success: bool,
}

// Example code of how we will implement threads to deal with our relay code
pub fn relay_main(rx: mpsc::Receiver<RelayMessagePacket>, tx: mpsc::Sender<RelayReturnPacket>) {
    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}

// Function that will control all of our tempurate control code, since it's just the relay controlling the AC
// RN, we are implementing it half assed.
pub fn temp_main(rx: mpsc::Receiver<RelayMessagePacket>, tx: mpsc::Sender<RelayReturnPacket>) {
    loop {
        let relay_status = rx.recv().unwrap();
        if (relay_status.relay_en) {
            // TODO
        }
    }
}

// Enumarted values representing
// The instructions that we are giving this thread.
#[derive(Clone, Debug)]
pub enum ClockControlMsg {
    CLOCK_EN,
    HEART_SLEEP,
    HEART_LOCK,
    HEART_LAMP,
    HEART_BEEP,
}
#[derive(Clone, Debug)]
pub struct HeartClockMessagePacket {
    pub msg_type: ClockControlMsg,
    pub val: bool,
}
pub struct HeartClockReturnPacket {
    pub request_status: bool,
}
pub fn heart_clock_control(
    rx: mpsc::Receiver<HeartClockMessagePacket>,
    tx: mpsc::Sender<HeartClockReturnPacket>,
) {
    // Strings containing the ip address and ports for our UDP control devices
    // These devices in particular don't use any particular data serialization format,
    // So we just send a preset array over the internet that represents a
    // command on the other side.
    let clock_control_ip_addr_port = String::from("192.168.1.24:4210");
    let heart_control_ip_addr_port = String::from("192.168.1.42:4250");

    // "object" of sorts that will let us control our clock
    let clock_control = udp_control::clock_control::ClockControl {
        socket: UdpSocket::bind("127.0.0.0:4050").expect("couldn't bind to address"),
        address_port: clock_control_ip_addr_port,
    };

    // objects of sorts that will let us control our heart.
    let heart_control = udp_control::heart_control::HeartControl {
        socket: UdpSocket::bind("127.0.0.0:4020").expect("Could not bind to address"),
        address_port: heart_control_ip_addr_port,
    };

    // Loop through everything.
    loop {
        let latest_msg = rx.recv().unwrap();
        // Which type of message are we dealing with
        match latest_msg.msg_type {
            CLOCK_EN => {
                if (latest_msg.val) {
                    clock_control.on();
                } else {
                    clock_control.off();
                }
            }
            HEART_LOCK => {
                if (latest_msg.val) {
                    heart_control.lock();
                } else {
                    heart_control.unlock();
                }
            }
            HEART_SLEEP => {
                if (latest_msg.val) {
                    // TODO
                } else {
                    // TODO
                }
            }
            HEART_LAMP => {
                if (latest_msg.val) {
                    heart_control.lamp_on();
                } else {
                    heart_control.lamp_off();
                }
            }
            HEART_BEEP => {
                heart_control.beep();
            }
        }
    }
}

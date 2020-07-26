// Rust's Files
use std::cell::{Cell, RefCell};
use std::net::UdpSocket;
use std::rc::Rc;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{thread, time};

// Since the structs for sending the information come from the "main device manager" thread
use crate::main_device_manager;

// Import scheduler program files
use crate::timer;

pub struct CloudSetupStruct {
    // Control of our main matrix control teensy board for the cli
    pub matrix_rx: mpsc::Sender<main_device_manager::MatrixMessagePacket>,

    // Control of our general purpose teensy board for the cli
    pub teensy_rx: mpsc::Sender<main_device_manager::TeensyMessagePacket>,

    // Control of our relayboards that deal with my thermometer.
    pub temp_rx: mpsc::Sender<main_device_manager::RelayMessagePacket>,

    pub hc_rx: mpsc::Sender<main_device_manager::HeartClockMessagePacket>,
}

pub fn cloud_main(cloud: CloudSetupStruct) {
    loop {
        // I figure 20 milliseconds between checking the cloud stuff is enough
        thread::sleep(Duration::from_millis(40));
    }
}

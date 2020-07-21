// Rust's Files
use std::net::UdpSocket;
use std::{thread, time};
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use std::rc::Rc;
use std::cell::{Cell, RefCell};

// Since the structs for sending the information come from the "main device manager" thread
use crate::main_device_manager;

// Import scheduler program files 
use crate::timer; 

pub struct CLISetupStruct{
    // Control of our main matrix control teensy board for the cli 
    pub matrix_rx: mpsc::Sender<main_device_manager::MatrixMessagePacket>,
    pub matrix_tx: mpsc::Receiver<main_device_manager::MatrixReturnPacket>,

    // Control of our general purpose teensy board for the cli 
    pub teensy_rx: mpsc::Sender<main_device_manager::TeensyMessagePacket>,
    pub teensy_tx: mpsc::Receiver<main_device_manager::TeensyReturnPacket>,

    // Control of our relayboards that deal with my thermometer. 
    pub temp_rx: mpsc::Sender<main_device_manager::RelayMessagePacket>,
    pub temp_tx: mpsc::Receiver<main_device_manager::RelayReturnPacket>, 

    pub hc_rx: mpsc::Sender<main_device_manager::HeartClockMessagePacket>,
    pub hc_tx: mpsc::Receiver<main_device_manager::HeartClockReturnPacket>
}

// function that contains our primary cli code
pub fn cli_main(cli_set: CLISetupStruct){
    loop{
        thread::sleep(Duration::from_millis(1000));
        let teensy_msg = main_device_manager::MatrixMessagePacket{debug: true};
        let matrix_msg = main_device_manager::TeensyMessagePacket{debug: true};
        
        // Send our messages
        cli_set.matrix_rx.send(teensy_msg);
        cli_set.teensy_rx.send(matrix_msg);
    }
}
// Rust's Files
use std::net::UdpSocket;
use std::{thread, time};
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use std::rc::Rc;
use std::cell::{Cell, RefCell};

// Since the structs for sending the information come from the "main device manager" thread
use crate::main_device_manager;

pub struct TimerSetupStruct{
    // Control of our main matrix control teensy board for the cli 
    pub matrix_rx: mpsc::Sender<main_device_manager::MatrixMessagePacket>,
    pub matrix_tx: mpsc::Receiver<main_device_manager::MatrixReturnPacket>,

    // Control of our general purpose teensy board for the cli 
    pub teensy_rx: mpsc::Sender<main_device_manager::TeensyMessagePacket>,
    pub teensy_tx: mpsc::Receiver<main_device_manager::TeensyReturnPacket>,

    // Control of our relayboards that deal with my thermometer. 
    pub temp_rx: mpsc::Sender<main_device_manager::RelayMessagePacket>,
    pub temp_tx: mpsc::Receiver<main_device_manager::RelayReturnPacket>
}

pub fn timer_main(timer_set: TimerSetupStruct){
    loop{
        thread::sleep(Duration::from_millis(1000));
    }
}
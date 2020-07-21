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
  
    // Control of our general purpose teensy board for the cli 
    pub teensy_rx: mpsc::Sender<main_device_manager::TeensyMessagePacket>,
  
    // Control of our relayboards that deal with my thermometer. 
    pub temp_rx: mpsc::Sender<main_device_manager::RelayMessagePacket>, 

    // Control of our heart and clock control devices. 
    pub hc_rx: mpsc::Sender<main_device_manager::HeartClockMessagePacket>
}

// Messaging used to control the timer subroutines. 
pub struct TimerMessagePacket{
    pub debug: bool
}
pub struct TimerReturnPacket{
    pub debug: bool
}
pub fn timer_main(timer_set: TimerSetupStruct){
    loop{
        
        thread::sleep(Duration::from_millis(1000));
        let msg = main_device_manager::HeartClockMessagePacket{
            msg_type: main_device_manager::ClockControlMsg::CLOCK_EN,
            val: true
        };
        timer_set.hc_rx.send(msg).unwrap();

        thread::sleep(Duration::from_millis(1000));
        let msg = main_device_manager::HeartClockMessagePacket{
            msg_type: main_device_manager::ClockControlMsg::CLOCK_EN,
            val: false
        };
        timer_set.hc_rx.send(msg).unwrap();
    }
}
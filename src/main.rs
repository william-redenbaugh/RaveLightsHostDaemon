// Rust's Files
use std::net::UdpSocket;
use std::{thread, time};
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use std::rc::Rc;
use std::cell::{Cell, RefCell};

// Incluide the standard libraries for input and output along with serial control.
use std::io::prelude::*;
use serial::prelude::*;

// Our UDP Control files 
// That let us control the devices on our wifi network
mod udp_control; 
use udp_control::{heart_control, matrix_control, clock_control, heaat_control, relay_control};

// Serial control files. 
// Lets us control devices on the local serial interface
mod serial_control; 
use serial_control::{teensy_control, strip_control};

// Serial module that we are using
extern crate serial;

// Protobuffer Messages
mod protobuf;
use protobuf::{messagedata, heaat_message, general_instructions, relay_msg};

// Whenever we deal with yahoo finance stuff it will get it's details and implementation from yahoo finance. 
extern crate yahoo_finance; 
use yahoo_finance::history;

// Importing the CLI libraries for dealing with keyboard inputs
mod cli; 
use cli::{cli_main};

// Importing the device manager libraries. 
mod device_manager;
use device_manager::main_device_manager;

// All of our timer integration features.
mod timer; 
use timer::timer_main;

// All of our cloud integration features 
mod cloud; 
use cloud::cloud_main; 

fn main() {   
    
    // Messaging for our matrix
    let (matrix_rx, rx) = mpsc::channel();
    let (tx, matrix_tx) = mpsc::channel();
    // Generate the thread to control our led matrix 
    let matrix_main_handle = thread::spawn(move || {
        main_device_manager::matrix_main(rx, tx);    
    });
    
    // Messaging for our teensy control
    let (teensy_rx, rx) = mpsc::channel();
    let (tx, teensy_tx) = mpsc::channel();

    // Generate the thread to control our main teensy 
    let teensy_main_handle = thread::spawn(move || {
        main_device_manager::teensy_main(rx, tx);
    });

    // Setting up the messaging for our tempurature channel
    let (temp_rx, rx) = mpsc::channel();
    let (tx, temp_tx) = mpsc::channel();
    let temp_main_handle = thread::spawn(move || {
        main_device_manager::temp_main(rx, tx);
    });

    // Struct that will let our timer execute scheduled orders
    // So we can have automated cloud functions
    let timer_setup = timer_main::TimerSetupStruct{
        matrix_rx: matrix_rx.clone(), 
        teensy_rx: teensy_rx.clone(), 
        temp_rx: temp_rx.clone()
    };

    // Generate the thread handler for our timer functions
    let timer_handle = std::thread::spawn(move || {
        timer_main::timer_main(timer_setup);
    });

    // Passing over the channels to the cli thread. 
    // So we can control our cloud integrated house 
    // using a command line interface. 
    let cli_setup = cli_main::CLISetupStruct{
        matrix_rx: matrix_rx.clone(),
        matrix_tx: matrix_tx,
        teensy_rx: teensy_rx.clone(),
        teensy_tx: teensy_tx,
        temp_rx: temp_rx.clone(), 
        temp_tx: temp_tx
    };
    
    // Setting up the threading for our CLI
    let cli_handle = thread::spawn(move || {
        cli_main::cli_main(cli_setup);   
    });
    
    // Struct that will allow our cloud integration thread
    // to issue commands to our device manipulation threads
    let cloud_setup = cloud_main::CloudSetupStruct{
        matrix_rx: matrix_rx, 
        teensy_rx: teensy_rx, 
        temp_rx: temp_rx
    };

    let cloud_handle = std::thread::spawn(move || {
        cloud_main::cloud_main(cloud_setup);
    });

    // Since we spawn everything in threads, 
    // We just wait on our threads to finish.
    // In theory since all of our threads 
    // 
    cli_handle.join();
    temp_main_handle.join();
    matrix_main_handle.join();
    teensy_main_handle.join();
    cloud_handle.join();
    timer_handle.join();
}
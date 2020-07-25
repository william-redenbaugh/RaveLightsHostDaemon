// Rust's Files
use std::net::UdpSocket;
use std::{thread, time};
use std::time::Duration;
use std::sync::{mpsc, Arc, Mutex};
use std::rc::Rc;
use std::cell::{Cell, RefCell};

// Since the structs for sending the information come from the "main device manager" thread
use crate::main_device_manager;

use chrono::{Datelike, Timelike, Utc};

// Struct that gives us all the information to control this stuff
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
// Should we ask anything of the timer message, that 
// Information should go here :0
pub struct TimerReturnPacket{
    pub debug: bool
}

// General purpose time event that 
// Let's us help schedule our events 
pub struct TimeEvent<T>{
    pub hour: u8, 
    pub minute: u8,
    pub day: u8,
    pub event_exec: bool,
    pub event: T
}
pub fn timer_main(timer_set: TimerSetupStruct){
    // Used for helping deal with our periodic functions 
    // So we make sure events only trigger once. 
    let mut timer_reset: bool = false; 

    // All events involving clock time events will be placed in this 
    // Vector array
    let mut clock_time_events = Vec::new();

    // Let's us deliver a clock off timer event 
    let clock_off_event = TimeEvent{
        hour: 2, 
        minute: 0, 
        day: 0,
        event: main_device_manager::clock_off_msg(),
        event_exec: false
    };

    // Message event for our clock on stuff
    let clock_on_event = TimeEvent{
        hour: 9, 
        minute: 20, 
        day: 0,
        event: main_device_manager::clock_on_msg(),
        event_exec: false
    };

    // Add our events to the array so we can run through and check for them 
    // To run 
    clock_time_events.push(clock_off_event);
    clock_time_events.push(clock_on_event);

    loop{
        // Get the latest time information needed to trigger our events
        let now = Utc::now();
        let (is_pm, hour) = now.hour12();
        let current_hour_utc = (((is_pm as u32) * 12) + hour) as u8;
        let current_hour = (current_hour_utc + 17) % 24;

        // Checking through all the events in the clock time stuff.
        for x in 0..clock_time_events.len(){
            // Checking the current time against when we want our time to trigger
            let hour_right: bool = clock_time_events[x].hour == current_hour;
            let minute_right: bool = clock_time_events[x].minute == (now.minute() as u8);
            
            // If all our flags match up, we deliver the message. 
            if(hour_right && minute_right &&(!clock_time_events[x].event_exec)){
                // Issues the message to the heart clock threads. 
                timer_set.hc_rx.send(clock_time_events[x].event.clone());
                // Resets time exec command 
                clock_time_events[x].event_exec = true; 
            }
        }

        thread::sleep(time::Duration::from_millis(2000));
        
        // So that we reset our timers every day.
        if(current_hour == 0 && now.minute() == 0 && !timer_reset){
            for x in 0..clock_time_events.len(){
                clock_time_events[x].event_exec = false; 
            }
            timer_reset = true; 
        }

        if(current_hour == 0 && now.minute() == 1){
            timer_reset = false;
        }
    }   
}
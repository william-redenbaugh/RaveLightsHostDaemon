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


fn multi_thread_one(){
    _test_heart();
}

fn multi_thread_two(){
    println!("Something else!");
    thread::sleep(Duration::from_millis(1000));
}

// Multithreading tests to help my write code: 
fn test_multithreading(){
    let counter = Arc::new(Mutex::new(0));

    let thread_one_counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        loop{
            {
                let mut num = thread_one_counter.lock().unwrap();
                *num += 1; 
                println!("Number from thread one: {}", *num);
            }
            thread::sleep(Duration::from_millis(500));
        }
    });

    let handle_one = thread::spawn(move || {
        loop{
            {
                let mut num = counter.lock().unwrap();
                *num += 1; 
                println!("Number from thread zero: {}", *num);
            }
            thread::sleep(Duration::from_millis(1000));
        }
    });

    handle.join().unwrap();    use std::io::prelude::*;
    use serial::prelude::*;
    let relay_addr_port = String::from("192.168.1.24::4040");
    let mut relay_ctrl = relay_control::RelayControl{
        socket: UdpSocket::bind("127.0.0.0:4050").expect("couldn't bind to address"),
        address_port: relay_addr_port
    };

    relay_ctrl.set(true);
}

fn _test_heaat(){
    let heaat_addr_port = String::from("192.168.1.24::4040");
    let mut heaat_ctrl = heaat_control::HeaatControl{
        socket: UdpSocket::bind("127.0.0.0:4050").expect("couldn't bind to address"),
        address_port: heaat_addr_port
    };

    heaat_ctrl.set(32, 32, 32, 12);
}

fn test_matrix(){
    // Address and port information for our Matrix
    let matrix_addr_port = String::from("192.168.1.9:4040");
    
    // Reference to our socket
    let socket = UdpSocket::bind("127.0.0.0:4050").expect("couldn't bind to address");
    let socket_ref = Rc::new(RefCell::new(socket));

    // Generates the array that we will save out matrix data in. 
    // On the heap, then ownership will be passed to MatrixController. 
    let matrix_arr: Vec<u8> = vec![0; 7000];
    let matrix_arr_cnv = matrix_arr.into_boxed_slice();
    
    // Creates matrix object with defined size, passing in the array 
    // Size of choice. 
    let mut matrix = matrix_control::MatrixControl{
        socket: Rc::clone(&socket_ref),
        address_port: matrix_addr_port,
        out_arr: matrix_arr_cnv, 
        x_len: 64, 
        y_len: 32 
    };

    matrix.begin();
    for x in 0..64{
        for y in 0..32{
            matrix.set_led(x, y, 100, 100, 100);
        }
    }

    matrix.update();
}

fn _test_teensy(){
    // Setup control with teensy over Serial port. 
    let mut p = serial::open("/dev/ttyAMA0").unwrap();
    let port_ref = Rc::new(RefCell::new(p));
    
    // TEENSY STRIP CONTROL BEGIN // 

    // Generates the array that we will save out matrix data in. 
    // On the heap, then ownership will be passed to MatrixController. 
    let matrix_arr: Vec<u8> = vec![0; 1000];
    let matrix_arr_cnv = matrix_arr.into_boxed_slice();

    let mut teensy = teensy_control::StripControl{
        out_arr: matrix_arr_cnv, 
        len: 288, 
        serial_port: port_ref
    };
    // TEENSY STRIP CONTROL END //
}

fn _test_strip(){
    
    // Generate Port with specific settings 
    let port = serial::open("/dev/ttyAMA0").unwrap();

    // Generates the array that we will save out matrix data in. 
    // On the heap, then ownership will be passed to MatrixController. 
    let matrix_arr: Vec<u8> = vec![0; 1000];
    let matrix_arr_cnv = matrix_arr.into_boxed_slice();

    let mut teensy = teensy_control::StripControl{
        out_arr: matrix_arr_cnv, 
        len: 288, 
        serial_port: port
    };

    teensy.begin_strip();
    teensy.set_led(12, 12, 12, 12);
    teensy.update_strip();
}

fn _test_clock(){

    // Construct our object. 
    let clk_control = clock_control::ClockControl{
                        off_cmd: [40, 40, 50, 65, 0], 
                        on_cmd: [40, 40, 50, 65, 1],
                        socket: UdpSocket::bind("192.168.1.2:4220").expect("couldn't bind to address")
                    };

    // Test functions. 
    clk_control.off();
    let one_seccond = time::Duration::from_millis(1000);
    thread::sleep(one_seccond);
    clk_control.on();
    thread::sleep(one_seccond);
}

fn _test_heart(){
    // Construct our heart. 
    let heart_control = heart_control::HeartControl{
        socket: UdpSocket::bind("192.168.1.2:4280").expect("couldn't bind to address")
    };

    heart_control.unlock();
    thread::sleep(Duration::from_millis(1000));
    heart_control.lamp_off();
    thread::sleep(Duration::from_millis(1000));
    heart_control.lamp_on();
    thread::sleep(Duration::from_millis(1000));
    heart_control.lock();
    thread::sleep(Duration::from_millis(1000));
    heart_control.toggle_lamp();
    thread::sleep(Duration::from_millis(1000));
    heart_control.beep();
    thread::sleep(Duration::from_millis(1000));
}
use core::num;
use std::time::Duration;

use serialport::SerialPort;
use crate::peripheral_control::shared;

pub struct USBStrip{
    data_arr: Vec<u8>,
    port: Box<dyn SerialPort>, 
    num_leds: usize
}

impl USBStrip {
    
    fn set_led(&mut self, index: usize, r: u8, g: u8, b: u8){
        let pixel_index = index * 3;
        self.data_arr[pixel_index] = r;
        self.data_arr[pixel_index + 1] = g;
        self.data_arr[pixel_index + 2] = b;
    }

    fn set_led_hsv(&mut self, index: usize, h: u8, s: u8, v: u8){
        let (r, g, b) = shared::hsv_to_rgb(h, s, v);
        self.set_led(index, r, g, b)
    }

    fn update(&mut self){
        
        // How many chunks
        let num_updates = 387;
        for x in 0..num_updates{
            let mut out_arr = vec![0; self.num_leds * 3 + 3];
            out_arr[0] = 128;
            out_arr[1] = 0;
            out_arr[2] = 2;

            // copy the data
            for y in 0..(128 * 3){
                out_arr[3 + y] = self.data_arr[x * 128 + y];
            }
            self.port.write(&out_arr);
        }

        let remainder_data = self.num_leds % 128;
        let mut out_arr = vec![0; self.num_leds * 3 + 3];
        out_arr[0] = remainder_data  as u8;
        out_arr[1] = 0;
        out_arr[2] = 2;

        // copy the data
        for y in 0..(remainder_data * 3){
            out_arr[3 + y] = self.data_arr[num_updates * 128 + y];
        }
        self.port.write(&out_arr).unwrap();

    }
}

fn top_uart_port() -> Option<Box<dyn SerialPort>>{
     // List available ports
     let available_ports = serialport::available_ports().expect("No ports found!");

     if available_ports.is_empty() {
         eprintln!("No serial ports found.");
         return None;
     }
 
     // Print available ports
     println!("Available ports:");
     for port in &available_ports {
         println!("{}", port.port_name);
     }
 
     // Select the first port in the list
     let port_name = &available_ports[0].port_name;
     println!("Using port: {}", port_name);
 
     // Set up serial port settings
     let baud_rate = 9600;
     let timeout = Duration::from_millis(1000);
 
     // Open the serial port
     let port = serialport::new(port_name, baud_rate)
         .timeout(timeout)
         .open()
         .expect("Failed to open serial port");        

    return Some(port);
}

fn init_usb_strip(num_leds: usize, port: Box<dyn SerialPort>)->USBStrip{
    let array = vec![0; num_leds* 3];

    return USBStrip { 
        data_arr: array, 
        port: (port), 
        num_leds: num_leds 
    };
}
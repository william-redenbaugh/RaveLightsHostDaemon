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
        
        let how_many_chunks = (self.num_leds * 3) / 384;
        let remainder = (self.num_leds * 3) % 384;

        // How many chunks
        for x in 0..how_many_chunks{
            let mut out_arr = vec![0; 387];
            out_arr[0] = 128;
            out_arr[1] = 0;
            out_arr[2] = 2;

            // copy the data
            for y in 0..(128){
                let index = (y * 3 + 3)  as usize;
                let data_arr_index = y * 3 + x * 384;
                out_arr[index] = self.data_arr[data_arr_index];
                out_arr[index + 1] = self.data_arr[data_arr_index + 1];
                out_arr[index + 2] = self.data_arr[data_arr_index + 2];
            }
            self.port.write(&out_arr).unwrap();
        }

        let mut out_arr = vec![0; remainder * 3 + 3];
        out_arr[0] = remainder  as u8;
        out_arr[1] = 0;
        out_arr[2] = 2;

        // copy the data
        for y in 0..(remainder){
            let index = (y * 3 + 3) as usize;
            let data_arr_index = y * 3 + how_many_chunks * 384;
            out_arr[index] = self.data_arr[data_arr_index];
            out_arr[index + 1] = self.data_arr[data_arr_index + 1];
            out_arr[index + 2] = self.data_arr[data_arr_index + 2];
        }

        self.port.write(&out_arr).unwrap();

    }
}

pub fn init_usb_strip(num_leds: usize, port: Box<dyn SerialPort>)->USBStrip{
    let array = vec![0; num_leds* 3];

    return USBStrip { 
        data_arr: array, 
        port: (port), 
        num_leds: num_leds 
    };
}

pub struct USBSMatrix{
    data_arr: Vec<u8>,
    width: usize, 
    length: usize,
    port: Box<dyn SerialPort>, 
}

impl USBSMatrix {
    
    pub fn set_led(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8){
        let pixel_index = (x * self.width + y) * 3 as usize;
        self.data_arr[pixel_index] = r;
        self.data_arr[pixel_index + 1] = g;
        self.data_arr[pixel_index + 2] = b;
    }

    pub fn set_led_hsv(&mut self, x: usize, y: usize, h: u8, s: u8, v: u8){
        let (r, g, b) = shared::hsv_to_rgb(h, s, v);
        self.set_led(x, y, r, g, b)
    }

    pub fn update(&mut self){
        
        let how_many_chunks = (self.width * self.length * 3) / 384;
        let remainder = (self.width * self.length * 3) % 384;

        // How many chunks
        for x in 0..how_many_chunks{
            let mut out_arr = vec![0; 387];
            out_arr[0] = 128;
            out_arr[1] = 0;
            out_arr[2] = 2;

            // copy the data
            for y in 0..(128){
                let index = (y * 3 + 3)  as usize;
                let data_arr_index = y * 3 + x * 384;
                out_arr[index] = self.data_arr[data_arr_index];
                out_arr[index + 1] = self.data_arr[data_arr_index + 1];
                out_arr[index + 2] = self.data_arr[data_arr_index + 2];
            }
            self.port.write(&out_arr).unwrap();
        }

        let mut out_arr = vec![0; remainder as usize * 3 + 3];
        out_arr[0] = remainder  as u8;
        out_arr[1] = 0;
        out_arr[2] = 2;

        // copy the data
        for y in 0..(remainder){
            let index = (y * 3 + 3) as usize;
            let data_arr_index = y * 3 + how_many_chunks * 384;
            out_arr[index] = self.data_arr[data_arr_index];
            out_arr[index + 1] = self.data_arr[data_arr_index + 1];
            out_arr[index + 2] = self.data_arr[data_arr_index + 2];
        }
        
        self.port.write(&out_arr).unwrap();
    }
}

pub fn init_usb_matrix(width: usize, length: usize, port: Box<dyn SerialPort>)->USBSMatrix{
    let array = vec![0; width * length * 3];

    return USBSMatrix { 
        data_arr: array, 
        port: (port), 
        width: width, 
        length: length
    };
}

pub fn top_uart_port() -> Option<Box<dyn SerialPort>>{
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
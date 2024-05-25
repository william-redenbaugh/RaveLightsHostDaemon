
use std::{net::UdpSocket, thread::sleep, time::Duration};

const LENGTH: usize = 16;
const WIDTH: usize = 16;
const BYTES_PER_LED: usize = 3;

pub struct UDPMatrix{
    data_arr: [u8; LENGTH * WIDTH * BYTES_PER_LED],
    socket: UdpSocket, 
    destination_addr: String
}

pub fn new_udp_matrix(address: String, dest_address: String) -> UDPMatrix{

    return UDPMatrix{
        data_arr: [0; LENGTH * WIDTH * BYTES_PER_LED], 
        socket: UdpSocket::bind(address).unwrap(), 
        destination_addr: dest_address
    };
}

fn hsv_to_rgb(h: u8, s: u8, v: u8) -> (u8, u8, u8) {
    let h = h as f32 / 255.0 * 360.0;
    let s = s as f32 / 255.0;
    let v = v as f32 / 255.0;

    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = match h {
        0.0..=60.0 => (c, x, 0.0),
        60.0..=120.0 => (x, c, 0.0),
        120.0..=180.0 => (0.0, c, x),
        180.0..=240.0 => (0.0, x, c),
        240.0..=300.0 => (x, 0.0, c),
        300.0..=360.0 => (c, 0.0, x),
        _ => (0.0, 0.0, 0.0),
    };

    let r = ((r + m) * 255.0).round() as u8;
    let g = ((g + m) * 255.0).round() as u8;
    let b = ((b + m) * 255.0).round() as u8;

    (r, g, b)
}

impl UDPMatrix{

    pub fn update(&mut self){
        let mut data: [u8; 387] = [0; 387];
        
        data[0] = 1; 
        data[1] = 0; 
        data[2] = 128;

        let mut value = 3;
        for x in 0..8{
            for y in 0..16{
                let matrix_index = (x * 16 + y) * 3;
                data[value] = self.data_arr[matrix_index];
                data[value + 1] = self.data_arr[matrix_index + 1];
                data[value + 2] = self.data_arr[matrix_index + 2];
                value = value + 3;
            }
        }

        self.socket.send_to(&data, &self.destination_addr).unwrap();

        data[0] = 2; 
        data[1] = 0; 
        data[2] = 128;
        
        value = 3;
        for x in 8..16{
            for y in 0..16{
                let matrix_index = (x * 16 + y) * 3;
                data[value] = self.data_arr[matrix_index];
                data[value + 1] = self.data_arr[matrix_index + 1];
                data[value + 2] = self.data_arr[matrix_index + 2];
                value = value + 3;
            }
        }
        
        self.socket.send_to(&data, &self.destination_addr).unwrap();
    }


    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8){
        let index = (x * 16 + y) * 3;
        self.data_arr[index] = r;
        self.data_arr[index + 1] = g;
        self.data_arr[index + 2] = b;
    }

    pub fn set_pixel_hsv(&mut self, x: usize, y: usize, h: u8, s: u8, v: u8){
        let (r, g, b) = hsv_to_rgb(h, s, v);
        self.set_pixel(x, y, r, g, b);
    }
    
}
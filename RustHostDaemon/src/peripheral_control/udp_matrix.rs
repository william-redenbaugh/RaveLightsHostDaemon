
use std::{net::UdpSocket};
use crate::peripheral_control::shared;

const BYTES_PER_LED: usize = 3;

pub struct UDPMatrix{
    data_arr:  Vec<u8>,
    socket: UdpSocket, 
    destination_addr: String, 
    width: usize, 
    length: usize
}

pub fn new_udp_matrix(address: String, dest_address: String, width: usize, length: usize) -> UDPMatrix{

    return UDPMatrix{
        data_arr: vec![0; width * length * BYTES_PER_LED],
        socket: UdpSocket::bind(address).unwrap(), 
        destination_addr: dest_address, 
        length: width, 
        width: width
    };
}

impl UDPMatrix{

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
            self.socket.send_to(&out_arr, &self.destination_addr).unwrap();

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

        self.socket.send_to(&out_arr, &self.destination_addr).unwrap();
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8){
        let index = (x * 16 + y) * 3;
        self.data_arr[index] = r;
        self.data_arr[index + 1] = g;
        self.data_arr[index + 2] = b;
    }

    pub fn set_pixel_hsv(&mut self, x: usize, y: usize, h: u8, s: u8, v: u8){
        let (r, g, b) = shared::hsv_to_rgb(h, s, v);
        self.set_pixel(x, y, r, g, b);
    }   
}

use std::net::UdpSocket;

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

impl UDPMatrix{

    pub fn update(&mut self){
        let mut data: [u8; 387] = [0; 387];
        
        data[0] = 0; 
        data[1] = 0; 
        data[2] = 128;

        for x in 0..8{
            for y in 0..16{
                let index = (x * 16 + y) * 3 + 3;
                let matrix_index = (x * 16 + y) * 3;
                data[index] = self.data_arr[matrix_index];
                data[index + 1] = self.data_arr[matrix_index + 1];
                data[index + 2] = self.data_arr[matrix_index + 2];
            }
        }
        self.socket.send_to(&data, &self.destination_addr).unwrap();

        data[0] = 0; 
        data[1] = 8; 
        data[2] = 128;

        for x in 8..16{
            for y in 0..16{
                let index = (x * 16 + y) * 3 + 3;
                let matrix_index = (x * 16 + y) * 3;
                data[index] = self.data_arr[matrix_index];
                data[index + 1] = self.data_arr[matrix_index + 1];
                data[index + 2] = self.data_arr[matrix_index + 2];
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
    
}
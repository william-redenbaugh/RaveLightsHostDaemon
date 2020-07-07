use std::net::UdpSocket;

pub struct MatrixControl{
    pub socket:UdpSocket, 
    pub out_arr: [u8; 6144]
}

impl MatrixControl {
    pub fn begin(&mut self){
        // Sets all values to zero, and pushes off the udp send command
        self.set_all_black();
        self.socket.send_to(&self.out_arr, "192.168.1.24:4210").expect("couldn't send data");
    }

    
    // Allows us to set all values to black
    pub fn set_all_black(&mut self){
        for x in 0..6144{
            self.out_arr[x] = 0; 
        }
    }
}
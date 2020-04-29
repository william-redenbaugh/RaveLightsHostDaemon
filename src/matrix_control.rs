use std::net::UdpSocket;

pub struct MatrixControl{
    pub socket:UdpSocket,
    // At must we support 500 neopixels, so 1500 bytes of
    //  information on the led array with some wiggle room 
    pub data_arr: Vec<u8>, 
    pub num_pixels: u16
}

impl MatrixControl {
    pub fn begin(&self){
        for _i in 0..(self.num_pixels * 3 + 99){
            self.data_arr.push(0);
        }
    }
}
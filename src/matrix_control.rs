use std::net::UdpSocket;

pub struct MatrixControl{
    pub socket:UdpSocket, 
    pub out_arr: [u8; 6144], 
    pub address_port: String
}

impl MatrixControl {
    // Since we are going to be modifying values to a class, this is how we do it!
    pub fn begin(&mut self){
        // Sets all values to zero, and pushes off the udp send command
        self.set_all_black();
        self.update();
    }
    
    pub fn set_led(&mut self, _x: u8, _y: u8, _r: u8, _g: u8, _b: u8){
        if (_x >= 64)  & (_y >= 32) {
            return;
        }

        let mut spot: usize = (_y * 64 + _x * 3 + 16).into(); 
       
        // Sets our out array spots
        self.out_arr[spot] = _r;
        spot = spot + 1; 
        self.out_arr[spot] = _g; 
        spot = spot + 1;
        self.out_arr[spot] = _b;
    }

    // Allows us to set all values to black
    pub fn set_all_black(&mut self){
        for x in 0..6144{
            self.out_arr[x] = 0; 
        }
    }

    pub fn update(&self){
        self.socket.send_to(&self.out_arr, &self.address_port).expect("couldn't send data");       
    }
}

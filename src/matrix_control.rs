use std::net::UdpSocket;

// Matrix Controller Object for the 64 by 32 panel. 
pub struct MatrixControl{
    pub socket:UdpSocket,  
    pub address_port: String,
    pub out_arr: Box<[u8]>,
    pub x_len: u8, 
    pub y_len: u8
}

impl MatrixControl{
    // Since we are going to be modifying values to a class, this is how we do it!
    pub fn begin(&mut self){
        // Sets all values to zero, and pushes off the udp send command
        self.set_all_black();
        self.update();
    }
    
    pub fn set_led(&mut self, _x: u8, _y: u8, _r: u8, _g: u8, _b: u8){
        if (_x >= self.x_len)  & (_y >= self.y_len) {
            return;
        }

        let mut spot: usize = ((_y * self.x_len + _x) * 3 + 16).into(); 
       
        // Sets our out array spots
        self.out_arr[spot] = _r;
        spot = spot + 1; 
        self.out_arr[spot] = _g; 
        spot = spot + 1;
        self.out_arr[spot] = _b;
    }

    // Allows us to set all values to black
    pub fn set_all_black(&mut self){
        for x in 0..(self.x_len * self.y_len * 3).into(){
            self.out_arr[x] = 0; 
        }
    }

    pub fn update(&self){
        self.socket.send_to(&self.out_arr, &self.address_port).expect("couldn't send data");       
    }
}
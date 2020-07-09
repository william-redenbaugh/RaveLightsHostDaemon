// The protobuffer crage that we are using
extern crate quick_protobuf; 

// Protobuffer writing module
use quick_protobuf::Writer;
// Standard UDP socket library
use std::net::UdpSocket;

// Message data for matrix stuff!
mod messagedata; 

// Matrix Controller Object for a variable sized panel 
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
        
        // Provides messagedata fields. 
        let val = messagedata::MessageData{
            message_size: (self.x_len as u32 * self.y_len as u32 * 3) as u32, 
            message_type: messagedata::mod_MessageData::MessageType::MATRIX_DATA, 
            return_message: false
        };

        {
            let mut out = Vec::new();
            let mut writer = Writer::new(&mut out);
            writer
                .write_message(&val)
                .expect("Message couldn't write properly");
            
            // Generally speaking the first 
            // Byte indicates the size of the array. 
            // But we don't require it for our purposes. 
            out.remove(0);
            let msg_fill = out.into_boxed_slice();

            // Fills in the message data that will
            // Indiciate what type of message this is!
            for x in 0.. (msg_fill.len()){
                self.out_arr[x] = msg_fill[x];
            }
        }
        
        // Sets all values to zero, and pushes off the udp send command
        self.update();
    }
    
    pub fn set_led(&mut self, _x: u8, _y: u8, _r: u8, _g: u8, _b: u8){
        if (_x >= self.x_len)  & (_y >= self.y_len) {
            return;
        }

        let mut spot: usize = ((_y as usize * self.x_len as usize + _x as usize) * 3 + 16); 
       
        // Sets our out array spots
        self.out_arr[spot] = _r;
        spot = spot + 1; 
        self.out_arr[spot] = _g; 
        spot = spot + 1;
        self.out_arr[spot] = _b;
    }

    pub fn update(&self){
        self.socket.send_to(&self.out_arr, &self.address_port).expect("couldn't send data");       
    }
}
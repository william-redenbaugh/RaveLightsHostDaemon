use std::net::UdpSocket;

// Object constructor udp control of the clock
pub struct ClockControl{
    pub socket:UdpSocket, 
    pub address_port: String
}

// Implementation for the UDP control of the clock
impl ClockControl{
    pub fn on(&self){
        let on_cmd: [u8; 5] = [40, 40, 50, 65, 0]; 
        self.socket.send_to(&on_cmd, &self.address_port).expect("couldn't send data");
    }

    pub fn off(&self){
        let off_cmd: [u8; 5] = [40, 40, 50, 65, 1];
        self.socket.send_to(&off_cmd, &self.address_port).expect("couldn't send data");
    }
}
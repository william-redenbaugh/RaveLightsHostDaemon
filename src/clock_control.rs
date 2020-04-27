use std::net::UdpSocket;

// Object constructor udp control of the clock
pub struct ClockControl{
    pub off_cmd: [u8; 5],
    pub on_cmd: [u8; 5], 
    pub socket:UdpSocket
}

// Implementation for the UDP control of the clock
impl ClockControl{
    pub fn on(&self){
        self.socket.send_to(&self.on_cmd, "192.168.1.24:4210").expect("couldn't send data");
    }

    pub fn off(&self){
        self.socket.send_to(&self.off_cmd, "192.168.1.24:4210").expect("couldn't send data");
    }
}
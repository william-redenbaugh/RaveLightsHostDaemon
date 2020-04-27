use std::net::UdpSocket;

// Object Reference for sending UDP commands to the heart. 
pub struct HeartControl{
    pub socket:UdpSocket
}

// Implimentation of the Object reference for sending UDP commands to the heart. 
impl HeartControl{
    // Allows us to remote lock the heart.
    pub fn lock(&self){
        let lock_cmd: [u8; 4] = [20, 20, 30, 90];
        self.socket.send_to(&lock_cmd, "192.168.1.42:4250").expect("couldn't send data");
    }

    // Allows us to remote unlock the heart. 
    pub fn unlock(&self){
        let lock_cmd: [u8; 4] = [20, 20, 30, 91];
        self.socket.send_to(&lock_cmd, "192.168.1.42:4250").expect("couldn't send data");
    }

    // Allows to turn on/off the lamp
    pub fn toggleLamp(&self){
        let cmd: [u8; 4] = [50, 50, 30, 92];
        self.socket.send_to(&cmd, "192.168.1.42:4250").expect("couldn't send data");
    }

    pub fn lampOn(&self){
        let cmd: [u8; 4] = [50, 50, 30, 91];
        self.socket.send_to(&cmd, "192.168.1.42:4250").expect("couldn't send data");
    }

    pub fn lampOff(&self){
        let cmd: [u8; 4] = [50, 50, 30, 90];
        self.socket.send_to(&cmd, "192.168.1.42:4250").expect("couldn't send data");
    }
    
    // Beep test the heart. 
    pub fn beep(&self){
        let cmd: [u8; 4] = [12, 12, 12, 12];
        self.socket.send_to(&cmd, "192.168.1.42:4250").expect("couldn't send data");
    }
}
use std::net::UdpSocket;
//use std::{thread, time};

pub struct ClockControl{
    pub off_cmd: [u8; 5],
    pub on_cmd: [u8; 5], 
    pub socket:UdpSocket
}

pub struct HeartControl{
    pub socket:UdpSocket
}

impl ClockControl{
    pub fn on(&self){
        self.socket.send_to(&self.on_cmd, "192.168.1.24:4210").expect("couldn't send data");
    }

    pub fn off(&self){
        self.socket.send_to(&self.off_cmd, "192.168.1.24:4210").expect("couldn't send data");
    }

    pub fn send_msg(&self, msg: String){
        //self.socket.send_to(&msg_start, "192.168.1.24:4210").expect("couldn't send data");
        //self.socket.send_to(msg.as_bytes(), "192.168.1.24:4210").expect("couldn't send data");
    }
}

impl HeartControl{
    pub fn lock(&self){
        let lock_cmd: [u8; 4] = [20, 20, 30, 90];
        self.socket.send_to(&lock_cmd, "192.168.1.42:4250").expect("couldn't send data");
    }

    pub fn unlock(&self){
        let lock_cmd: [u8; 4] = [20, 20, 30, 91];
        self.socket.send_to(&lock_cmd, "192.168.1.42:4250").expect("couldn't send data");
    }

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

    pub fn beep(&self){
        let cmd: [u8; 4] = [12, 12, 12, 12];
        self.socket.send_to(&cmd, "192.168.1.42:4250").expect("couldn't send data");
    }
}
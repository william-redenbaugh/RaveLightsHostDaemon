// The protobuffer crage that we are using
extern crate quick_protobuf; 

// Protobuffer writing module
use quick_protobuf::Writer;

// Standard UDP socket library
use std::net::UdpSocket;

// Crate that contains our clock programming data. 
use crate::{
        messagedata, 
        heart_program_control
    };

// Object Reference for sending UDP commands to the heart. 
pub struct HeartControl{
    pub socket:UdpSocket, 
    pub address_port: String, 
    pub sleep_mode: bool, 
    pub lock_mode: bool, 
    pub lamp_mode: bool
}

pub fn new_heart_control(udp_serv: UdpSocket, address_port: String) -> HeartControl{
    let heart_obj = HeartControl{
        socket: udp_serv, 
        address_port: address_port, 
        sleep_mode: true, 
        lock_mode: true, 
        lamp_mode: false
    };
    return heart_obj;
}

fn command_heart(sleep_mode: bool, lock_mode: bool, lamp_mode: bool) -> Box<[u8]>{
    let mut out_arr = Vec::new();
    let mut heart_instr_out = Vec::new();
    // Adding our clock program instructions
    {
        let msg = heart_program_control::HeartControlData{
            en_sleep_mode: sleep_mode, 
            en_lamp: lamp_mode, 
            en_lock_mode: lock_mode
        };
        let mut writer = Writer::new(&mut heart_instr_out);
        writer
            .write_message(&msg)
            .expect("Message could not write properly :(");
        heart_instr_out.remove(0);            
    }
    
    {
        // Provides messagedata fields. 
        let val = messagedata::MessageData{
            message_size: (heart_instr_out.len()) as u32, 
            message_type: messagedata::mod_MessageData::MessageType::HEART_DATA, 
            return_message: false
        };
        
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        writer
            .write_message(&val)
            .expect("Message couldn't write properly");

        // Generally speaking the first 
        // Byte indicates the size of the array. 
        // But we don't require it for our purposes. 
        out.remove(0);
        
        // Fills in the message data that will
        // Indiciate what type of message this is!
        for x in 0.. (out.len()){
            out_arr.push(out[x]);
        }
        for x in 0.. (16 - out.len()){
            out_arr.push(0);
        }
    }
    // Fills in the message data that will
    // Indiciate what type of message this is!
    for x in 0.. (heart_instr_out.len()){
        out_arr.push(heart_instr_out[x]);
    }

    // Return the out array
    return out_arr.into_boxed_slice();
}

// Implimentation of the Object reference for sending UDP commands to the heart. 
impl HeartControl{
    // Allows us to remote lock the heart.
    pub fn lock_legacy(&self){
        let lock_cmd: [u8; 4] = [20, 20, 30, 90];
        self.socket.send_to(&lock_cmd, &self.address_port).expect("Couldn't send heart control data :(");
    }

    // Allows us to remote unlock the heart. 
    pub fn unlock_legacy(&self){
        let lock_cmd: [u8; 4] = [20, 20, 30, 91];
        self.socket.send_to(&lock_cmd, &self.address_port).expect("Couldn't send heart control data :(");
    }

    // Allows to turn on/off the lamp
    pub fn toggle_lamp_legacy(&self){
        let cmd: [u8; 4] = [50, 50, 30, 92];
        self.socket.send_to(&cmd, &self.address_port).expect("Couldn't send heart control data :(");
    }

    pub fn lamp_on_legacy(&self){
        let cmd: [u8; 4] = [50, 50, 30, 91];
        self.socket.send_to(&cmd, &self.address_port).expect("Couldn't send heart control data :(");
    }

    pub fn lamp_off_legacy(&self){
        let cmd: [u8; 4] = [50, 50, 30, 90];
        self.socket.send_to(&cmd, &self.address_port).expect("Couldn't send heart control data :(");
    }
    
    // Beep test the heart. 
    pub fn beep(&self){
        let cmd: [u8; 4] = [12, 12, 12, 12];
        self.socket.send_to(&cmd, &self.address_port).expect("Couldn't send heart control data :(");
    }

    // Allows us to remote lock the heart.
    pub fn lock(&mut self){
        self.lock_mode = true; 
        let lock_cmd = command_heart(self.sleep_mode, true, self.lamp_mode);
        self.socket.send_to(&lock_cmd, &self.address_port).expect("Couldn't send heart control data :(");
    }

    // Allows us to remote unlock the heart. 
    pub fn unlock(&mut self){
        self.lock_mode = false; 
        let lock_cmd = command_heart(self.sleep_mode, false, self.lamp_mode);
        self.socket.send_to(&lock_cmd, &self.address_port).expect("Couldn't send heart control data :(");
    }

    // Allows to turn on/off the lamp
    pub fn toggle_lamp(&mut self){
        self.lamp_mode = !self.lamp_mode; 
        let lamp_cmd = command_heart(self.sleep_mode, self.lock_mode, self.lamp_mode);
        self.socket.send_to(&lamp_cmd, &self.address_port).expect("Couldn't send heart control data :(");
    }

    pub fn lamp_on(&mut self){
        self.lamp_mode = true; 
        let lamp_cmd = command_heart(self.sleep_mode, self.lock_mode, self.lamp_mode);
        self.socket.send_to(&lamp_cmd, &self.address_port).expect("Couldn't send heart control data :(");
    }

    pub fn lamp_off(&mut self){
        self.lamp_mode = false; 
        let lamp_cmd = command_heart(self.sleep_mode, self.lock_mode, self.lamp_mode);
        self.socket.send_to(&lamp_cmd, &self.address_port).expect("Couldn't send heart control data :(");
    }

    pub fn sleep(&mut self){
        self.sleep_mode = true; 
        let sleep_cmd = command_heart(self.sleep_mode, self.lock_mode, self.lamp_mode);
        self.socket.send_to(&sleep_cmd, &self.address_port).expect("Couldn't send heart control data :(");
    }

    pub fn wake(&mut self){
        self.sleep_mode = false; 
        let wake_cmd = command_heart(self.sleep_mode, self.lock_mode, self.lamp_mode);
        self.socket.send_to(&wake_cmd, &self.address_port).expect("Couldn't send heart control data :(");
    }
}
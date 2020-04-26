use std::net::UdpSocket;
use std::{thread, time};
mod clock_control;

fn main() {
    //test_clock();

    test_heart();
}

fn test_clock(){
    let clk_control = clock_control::ClockControl{
                        off_cmd: [40, 40, 50, 65, 0], 
                        on_cmd: [40, 40, 50, 65, 1],
                        socket: UdpSocket::bind("192.168.1.2:4220").expect("couldn't bind to address")
                    };
    
    clk_control.off();
    let one_seccond = time::Duration::from_millis(1000);
    thread::sleep(one_seccond);
    clk_control.on();
    thread::sleep(one_seccond);
    
    let hello = "Hello World!";
    clk_control.send_msg(hello.to_string());
}

fn test_heart(){
    let heart_control = clock_control::HeartControl{
        socket: UdpSocket::bind("192.168.1.2:4280").expect("couldn't bind to address")
    };

    //heart_control.lock();
    
    let one_seccond = time::Duration::from_millis(1000);
    thread::sleep(one_seccond);
    
    heart_control.unlock();

    heart_control.lampOff();
}
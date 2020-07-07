use std::net::UdpSocket;
use std::{thread, time};
mod clock_control;
mod heart_control;
mod matrix_control; 

fn main() {
    test_matrix();
    test_heart();
    test_clock();
}

fn test_matrix(){
    // Address and port information for our Matrix
    let matrix_addr_port = String::from("192.168.1.8:4220");
    
    // Generates the array that we will save out matrix data in. 
    // On the heap, then ownership will be passed to MatrixController. 
    let matrix_arr: Vec<u8> = vec![0; 6160];
    let matrix_arr_cnv = matrix_arr.into_boxed_slice();
    
    // Creates matrix object with defined size, passing in the array 
    // Size of choice. 
    let mut matrix = matrix_control::MatrixControl{
        socket: UdpSocket::bind(&matrix_addr_port).expect("couldn't bind to address"),
        address_port: matrix_addr_port,
        out_arr: matrix_arr_cnv, 
        x_len: 64, 
        y_len: 32 
    };

    matrix.begin();
    
    for x in 0..64{
        for y in 0..32{
            matrix.set_led(x, y, 100, 100, 100);
        }
    }

    matrix.update();
    matrix.set_all_black();
    
}

fn test_clock(){

    // Construct our object. 
    let clk_control = clock_control::ClockControl{
                        off_cmd: [40, 40, 50, 65, 0], 
                        on_cmd: [40, 40, 50, 65, 1],
                        socket: UdpSocket::bind("192.168.1.2:4220").expect("couldn't bind to address")
                    };

    // Test functions. 
    clk_control.off();
    let one_seccond = time::Duration::from_millis(1000);
    thread::sleep(one_seccond);
    clk_control.on();
    thread::sleep(one_seccond);
}

fn test_heart(){
    // Construct our heart. 
    let heart_control = heart_control::HeartControl{
        socket: UdpSocket::bind("192.168.1.2:4280").expect("couldn't bind to address")
    };

    // Test heart parameters. 
    //heart_control.lock();
    
    let one_seccond = time::Duration::from_millis(1000);
    thread::sleep(one_seccond);
    
    heart_control.unlock();
    heart_control.lamp_off();
    heart_control.lamp_on();
    heart_control.lock();
    heart_control.toggle_lamp();
    heart_control.beep();
}
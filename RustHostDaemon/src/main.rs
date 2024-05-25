mod udp_matrix;
mod audio_input_processing;
use std::thread::sleep;
use std::time::Duration;



fn main() {

    let mut udp_matrix =udp_matrix::new_udp_matrix(String::from("192.168.3.234:4200"), String::from("192.168.3.249:34254"));
    
    udp_matrix.set_pixel(0, 0, 100, 100, 100);
    udp_matrix.update();
    
    //let audio_data = audio_input_processing::initialize_audio_pipeline();

    let mut val:u32 = 0;
    loop{
        for x in 0..16{
            for y in 0..16{
                val = val + 20;
                val = val % 255;

                udp_matrix.set_pixel_hsv(x, y, val as u8, 255, 255)
            }
        }
        udp_matrix.update();
        println!("updating...");
        sleep(Duration::from_millis(10));
    }
}

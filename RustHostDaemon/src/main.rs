mod audio_pipelines;
mod peripheral_control;

fn main() {

    let mut udp_matrix = peripheral_control::udp_matrix::new_udp_matrix(
        String::from("192.168.3.234:4200"), 
        String::from("192.168.3.249:34254"),
        16, 
        16
    );
    
    udp_matrix.set_pixel(0, 0, 100, 100, 100);
    udp_matrix.update();
    
    let audio_data = audio_pipelines::audio_input_processing::initialize_audio_pipeline();
    let mut vals: [u8; 16] = [0; 16];
    loop{

        let data = audio_data.recv().unwrap();
        for x in 0..16{

            let mut val = 0; 
            for j in 0..16{
                let mut sub_val;
                sub_val = (data[x * 16 + j].re).abs() as i32;

                sub_val = sub_val/30000;
                val = (val + sub_val)/2;
            }

            if val > 16{
                val = 16;
            }
            
            if val > vals[x] as i32 {
                vals[x] = val as u8;
            }
            for y in 0..vals[x]{
                udp_matrix.set_pixel_hsv(x, y as  usize, 255 - vals[x] * 6, 255, 255)
            }
            for y in vals[x]..16{
                udp_matrix.set_pixel(x, y as usize, 0, 0, 0);
            }
            if vals[x] > 0{
                vals[x] = vals[x] - 1;
            }
        }
        udp_matrix.update();
    }
}


use std::thread;
use std::thread::sleep;
use std::time::Duration;
use rustfft::{FftPlanner, num_complex::Complex};
use std::sync::mpsc::{self, Sender, Receiver};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn fft_processing_thread(rx: Receiver<Vec<i16>>, tx_complex_data: Sender<Vec<Complex<i16>>>){

    
    loop{
        let data = rx.recv().unwrap();

        let mut planner: FftPlanner<i16> = FftPlanner::new();
        let fft = planner.plan_fft_forward(data.len());
        
        let mut complex_data: Vec<Complex<i16>> = data.into_iter().map(|x| Complex::new(x, 0)).collect();

        fft.process(&mut complex_data);
        tx_complex_data.send(complex_data).unwrap();
    }
}

fn microphone_input_thread(tx: Sender<Vec<i16>>){

    // Set up the host and default input device.
    let host = cpal::default_host();
    let device = host.default_input_device().expect("failed to find input device");

    let config = device
    .default_input_config()
    .expect("Failed to get default input config");
    println!("Default input config: {:?}", config);

    loop{
        let duration = Duration::from_millis(10);

        // Wrap the Duration in an Option
        let tx_copy = tx.clone();
        // Create and start the input stream.
        let stream = device.build_input_stream(
            &config.config(),
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                println!("Length: {}", data.len());
                tx_copy.send(data.to_vec()).unwrap();
            },
            move |err| {
                eprintln!("An error occurred on the input audio stream: {}", err);
            }, 
            Some(duration)
        ).expect("Failed to build input stream");

        stream.play().unwrap();        
        sleep(duration);
    }
}

pub fn initialize_audio_pipeline() -> Receiver<Vec<Complex<i16>>>{
    // Create pipes
    let (tx_complex_data, rx_complex_data) = mpsc::channel();
    let (tx, rx): (Sender<Vec<i16>>, Receiver<Vec<i16>>) = mpsc::channel();
    
    // Spawn thread for processing the FFT data
    
    thread::spawn(move || fft_processing_thread(rx, tx_complex_data));
    thread::spawn(move || microphone_input_thread(tx));
    return rx_complex_data;
}

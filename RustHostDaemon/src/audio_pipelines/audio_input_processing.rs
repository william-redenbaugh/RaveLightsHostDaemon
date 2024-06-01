
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use cpal::SampleRate;
use rustfft::{FftPlanner, num_complex::Complex};
use std::sync::mpsc::{self, Sender, Receiver};
use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}, SupportedStreamConfigRange};


fn fft_processing_thread(rx: Receiver<Vec<i16>>, tx_complex_data: Sender<Vec<Complex<f32>>>){
    loop{
        let mut data = rx.recv().unwrap();
        for l in 0..512{
            data[l] = data[l]/2;   
        }
        data.truncate(512);
        let mut planner: FftPlanner<f32> = FftPlanner::new();
        let fft = planner.plan_fft_forward(data.len());
        
        let mut complex_data: Vec<Complex<f32>> = data.into_iter().map(|x| Complex::new(x as f32, x as f32)).collect();
        fft.process(&mut complex_data);

        tx_complex_data.send(complex_data).unwrap();
    }
}

fn fft_inverse_processing_thread(tx_complex_data: Sender<Vec<Complex<f32>>>, incoming_unfiltered_fft_data: Receiver<Vec<Complex<f32>>>){
    loop{

        let mut incoming_data = incoming_unfiltered_fft_data.recv().unwrap();
        // Apply inverse FFT to get the smoothed data
        let mut ifft_output = vec![Complex::new(0.0, 0.0); 512];
        let mut planner = FftPlanner::new();
        let ifft = planner.plan_fft(512, rustfft::FftDirection::Inverse);

        ifft.process_with_scratch(&mut incoming_data, &mut ifft_output);
        tx_complex_data.send(ifft_output).unwrap();
    }
}

fn microphone_input_thread(tx: Sender<Vec<i16>>){

    // Set up the host and default input device.
    let host = cpal::default_host();
    let device = host.default_input_device().expect("failed to find input device");

    let config_range = SupportedStreamConfigRange::new(1, 
        SampleRate(17200), 
        SampleRate(17200), 
        cpal::SupportedBufferSize::Range {
            min: (512), 
            max: (1024)} , 
            cpal::SampleFormat::I16);
    let config = config_range.with_max_sample_rate();
    
    device
    .default_input_config()
    .expect("Failed to get default input config");
    println!("Default input config: {:?}", config);
    // Wrap the Duration in an Option
    let tx_copy = tx.clone();
    // Create and start the input stream.
    let stream = device.build_input_stream(
        &config.config(),
        move |data: &[i16], _: &cpal::InputCallbackInfo| {
            //println!("Length: {}", data.len());
            if data.len() < 512 {
                return;
            }
            tx_copy.send(data.to_vec()).unwrap();
        },
        move |err| {
            eprintln!("An error occurred on the input audio stream: {}", err);
        }, 
        None
    ).expect("Failed to build input stream");

    stream.play().unwrap();  

    loop{
        sleep(Duration::from_secs(5));
    }
}

pub fn initialize_audio_pipeline() -> Receiver<Vec<Complex<f32>>>{
    // Create pipes
    let (tx_complex_data, rx_complex_data) = mpsc::channel();
    let (tx, rx): (Sender<Vec<i16>>, Receiver<Vec<i16>>) = mpsc::channel();
    
    let (tx_normalized_data, rx_nromalized_data) = mpsc::channel();
    // Spawn thread for processing the FFT data
    
    thread::spawn(move || fft_inverse_processing_thread(tx_normalized_data, rx_complex_data));
    thread::spawn(move || fft_processing_thread(rx, tx_complex_data));
    thread::spawn(move || microphone_input_thread(tx));
    return rx_nromalized_data;
}

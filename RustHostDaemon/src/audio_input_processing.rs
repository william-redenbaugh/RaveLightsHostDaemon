
use std::thread;
use std::time::Duration;
use rustfft::{FftPlanner, num_complex::Complex};
use lazy_static::lazy_static;
use std::sync::mpsc::{self, Sender, Receiver};
use std::sync::Mutex;

static mut SENDER: Option<Mutex<Sender<Vec<i16>>>> = None;
static mut RECEVIER: Option<Mutex<Receiver<Vec<i16>>>> = None;

fn init_channel() -> Receiver<Vec<i16>>{
    let (tx, rx): (Sender<Vec<i16>>, Receiver<Vec<i16>>) = mpsc::channel();
    unsafe {
        SENDER = Some(Mutex::new(tx));
    }

    return rx;
}

fn read_callback(stream: &mut soundio::InStreamReader) {
    let frame_count_max = stream.frame_count_max();
    if let Err(e) = stream.begin_read(frame_count_max) {
        println!("Error reading from stream: {}", e);
        return;
    }
    
    let mut buffer: Vec<i16> = vec![0; stream.frame_count()];
    for f in 0..stream.frame_count() {
        buffer[f] = stream.sample::<i16>(0, f);
    }

    unsafe {
        if let Some(ref sender) = SENDER {
            let mut sender = sender.lock().unwrap();
            sender.send(buffer).unwrap();
        }
    }
}

fn fft_processing_thread(tx_complex_data: Sender<Vec<Complex<i16>>>){
    let mut ctx = soundio::Context::new();
    ctx.set_app_name("FFT Processing Audio Module");
    ctx.connect().unwrap();
    ctx.flush_events();

    let mut dev = ctx.input_devices().unwrap();
    
    if(dev.len() <= 0){
        return
    }

    let rx = init_channel();

    let mut input_stream = dev[0].open_instream(
        44100,
        soundio::Format::S16LE,
        soundio::ChannelLayout::get_builtin(soundio::ChannelLayoutId::Mono),
        2.0,
        read_callback,
        None::<fn()>,
        None::<fn(soundio::Error)>,
    ).unwrap();
    input_stream.start().unwrap();

    loop{
        let data = rx.recv().unwrap();

        let mut planner: FftPlanner<i16> = FftPlanner::new();
        let fft = planner.plan_fft_forward(data.len());
        
        let mut complex_data: Vec<Complex<i16>> = data.into_iter().map(|x| Complex::new(x, 0)).collect();

        fft.process(&mut complex_data);
        tx_complex_data.send(complex_data);
    }
}

pub fn initialize_audio_pipeline() -> Receiver<Vec<Complex<i16>>>{
    let (tx_complex_data, rx_complex_data) = mpsc::channel();
    thread::spawn(move || fft_processing_thread(tx_complex_data));
    return rx_complex_data;
}

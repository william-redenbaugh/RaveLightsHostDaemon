mod udp_matrix;
mod audio_input_processing;

fn main() {
    let audio_data = audio_input_processing::initialize_audio_pipeline();
}

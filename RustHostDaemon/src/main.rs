pub mod matrix;

fn main() -> std::io::Result<()> {
    // Create an instance of LedMatrixController with a 32x64 LED matrix
    let mut led_controller = matrix::LedMatrixController::new("192.168.4.72:5000", 32, 128)?;

    // Set LED color at position (5, 10) to red (255, 0, 0)
    led_controller.set_led(5, 10, 255, 0, 0);

    // Update and send LED matrix data using the implemented function
    led_controller.update_matrix()?;

    Ok(())
}
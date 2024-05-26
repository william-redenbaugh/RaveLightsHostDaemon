
fn init_usb_strip(){
     // List available ports
     let available_ports = serialport::available_ports().expect("No ports found!");

     if available_ports.is_empty() {
         eprintln!("No serial ports found.");
         return Ok(());
     }
 
     // Print available ports
     println!("Available ports:");
     for port in &available_ports {
         println!("{}", port.port_name);
     }
 
     // Select the first port in the list
     let port_name = &available_ports[0].port_name;
     println!("Using port: {}", port_name);
 
     // Set up serial port settings
     let baud_rate = 9600;
     let timeout = Duration::from_millis(1000);
 
     // Open the serial port
     let mut port = serialport::new(port_name, baud_rate)
         .timeout(timeout)
         .open()
         .expect("Failed to open serial port");        
}
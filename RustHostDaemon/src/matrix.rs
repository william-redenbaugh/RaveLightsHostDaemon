use std::net::UdpSocket;
use std::net::SocketAddr;

pub struct LedMatrixController {
    socket: UdpSocket,
    buffer: Vec<u8>,
    target_addr: Box<str>,
    width: usize,
    height: usize,
}

impl LedMatrixController {
    pub fn new(address: &str, width: usize, height: usize) -> std::io::Result<Self> {
        let socket = UdpSocket::bind("192.168.4.72:5000")?;
        let buffer_size = width * height * 3; // 3 bytes per LED (RGB)
        let buffer = vec![0u8; buffer_size];
        Ok(Self {
            socket,
            buffer,
            address,
            width,
            height,
        })
    }

    pub fn update_matrix(&mut self) -> std::io::Result<usize> {
        let dest_addr: SocketAddr = "raspberrypi.local:6060".parse().expect("Failed to parse address");
        self.socket.send_to(&self.buffer, &self.target)
    }

    pub fn set_led(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        // Ensure the provided coordinates are within the LED matrix boundaries
        if x >= self.width || y >= self.height {
            panic!("Coordinates out of bounds for LED matrix");
        }

        // Calculate the index in the buffer based on x and y coordinates
        let index = ((y * self.width) + x) * 3;

        // Set the color of the LED at the calculated index
        self.buffer[index] = r;
        self.buffer[index + 1] = g;
        self.buffer[index + 2] = b;
    }
}
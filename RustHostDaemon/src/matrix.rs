use std::net::UdpSocket;
use std::net::SocketAddr;
use image::{GenericImageView};
use gif::{Frame, Decoder};

pub struct LedMatrixController {
    socket: UdpSocket,
    buffer: Vec<u8>,
    target_addr: SocketAddr,
    width: usize,
    height: usize,
}

impl LedMatrixController {
    pub fn new(target_addr: SocketAddr, width: usize, height: usize) -> std::io::Result<Self> {
        let socket = UdpSocket::bind("192.168.4.72:5000")?;
        let buffer_size = width * height * 3; // 3 bytes per LED (RGB)
        let buffer = vec![0u8; buffer_size];
        Ok(Self {
            socket,
            buffer,
            target_addr,
            width,
            height,
        })
    }

    pub fn update_matrix(&mut self) -> std::io::Result<usize> {
        self.socket.send_to(&self.buffer, &self.target_addr)
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

    // Method to draw a vertical line on the LED matrix
    pub fn draw_vertical_line(&mut self, x: usize, y_start: usize, y_end: usize, r: u8, g: u8, b: u8) {
        if x >= self.width || y_start >= self.height || y_end >= self.height {
            panic!("Coordinates out of bounds for drawing a vertical line");
        }

        for y in y_start..=y_end {
            self.set_led(x, y, r, g, b);
        }
    }

    // Method to draw a horizontal line on the LED matrix
    pub fn draw_horizontal_line(&mut self, x_start: usize, x_end: usize, y: usize, r: u8, g: u8, b: u8) {
        if x_start >= self.width || x_end >= self.width || y >= self.height {
            panic!("Coordinates out of bounds for drawing a horizontal line");
        }

        for x in x_start..=x_end {
            self.set_led(x, y, r, g, b);
        }
    }

    // Method to draw a line from point one to point two using Bresenham's line algorithm
    pub fn draw_line(&mut self, x0: usize, y0: usize, x1: usize, y1: usize, r: u8, g: u8, b: u8) {
        if x0 >= self.width || y0 >= self.height || x1 >= self.width || y1 >= self.height {
            panic!("Coordinates out of bounds for drawing a line");
        }

        let mut x = x0 as isize;
        let mut y = y0 as isize;
        let dx = (x1 as isize - x0 as isize).abs();
        let dy = -(y1 as isize - y0 as isize).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        while x != x1 as isize || y != y1 as isize {
            self.set_led(x as usize, y as usize, r, g, b);
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }

    // Method to draw a circle on the LED matrix
    pub fn draw_circle(&mut self, xc: usize, yc: usize, radius: usize, r: u8, g: u8, b: u8) {
        if xc >= self.width || yc >= self.height {
            panic!("Coordinates out of bounds for drawing a circle");
        }

        let mut x = radius as isize - 1;
        let mut y = 0;
        let mut dx = 1;
        let mut dy = 1;
        let mut err = dx - (radius << 1) as isize;

        while x >= y {
            self.set_led((xc as isize + x) as usize, (yc as isize + y) as usize, r, g, b);
            self.set_led((xc as isize + y) as usize, (yc as isize + x) as usize, r, g, b);
            self.set_led((xc as isize - y) as usize, (yc as isize + x) as usize, r, g, b);
            self.set_led((xc as isize - x) as usize, (yc as isize + y) as usize, r, g, b);
            self.set_led((xc as isize - x) as usize, (yc as isize - y) as usize, r, g, b);
            self.set_led((xc as isize - y) as usize, (yc as isize - x) as usize, r, g, b);
            self.set_led((xc as isize + y) as usize, (yc as isize - x) as usize, r, g, b);
            self.set_led((xc as isize + x) as usize, (yc as isize - y) as usize, r, g, b);

            if err <= 0 {
                y += 1;
                err += dy;
                dy += 2;
            }
            if err > 0 {
                x -= 1;
                dx += 2;
                err += dx - (radius << 1) as isize;
            }
        }
    }

    // Method to draw a triangle on the LED matrix
    pub fn draw_triangle(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, x3: usize, y3: usize, r: u8, g: u8, b: u8) {
        self.draw_line(x1, y1, x2, y2, r, g, b);
        self.draw_line(x2, y2, x3, y3, r, g, b);
        self.draw_line(x3, y3, x1, y1, r, g, b);
    }

    // Method to draw a rectangle on the LED matrix
    pub fn draw_rectangle(&mut self, x1: usize, y1: usize, x2: usize, y2: usize, r: u8, g: u8, b: u8) {
        self.draw_horizontal_line(x1, x2, y1, r, g, b);
        self.draw_horizontal_line(x1, x2, y2, r, g, b);
        self.draw_vertical_line(x1, y1, y2, r, g, b);
        self.draw_vertical_line(x2, y1, y2, r, g, b);
    }

    // Method to draw a sine wave on the LED matrix
    pub fn draw_sine_wave(&mut self, amplitude: f32, frequency: f32, phase_shift: f32, r: u8, g: u8, b: u8) {
        let mut x_prev = 0;
        let mut y_prev = (self.height as f32 / 2.0 + (amplitude * (frequency * 0.0 + phase_shift)).sin()) as usize;

        for i in 1..self.width {
            let y = (self.height as f32 / 2.0 + (amplitude * (frequency * i as f32 + phase_shift)).sin()) as usize;
            self.draw_line(x_prev, y_prev, i, y, r, g, b);
            x_prev = i;
            y_prev = y;
        }
    }

    // Method to draw a cosine wave on the LED matrix
    pub fn draw_cosine_wave(&mut self, amplitude: f32, frequency: f32, phase_shift: f32, r: u8, g: u8, b: u8) {
        let mut x_prev = 0;
        let mut y_prev = (self.height as f32 / 2.0 + (amplitude * (frequency * 0.0 + phase_shift)).cos()) as usize;

        for i in 1..self.width {
            let y = (self.height as f32 / 2.0 + (amplitude * (frequency * i as f32 + phase_shift)).cos()) as usize;
            self.draw_line(x_prev, y_prev, i, y, r, g, b);
            x_prev = i;
            y_prev = y;
        }
    }

    // Method to draw a tangent wave on the LED matrix
    pub fn draw_tangent_wave(&mut self, amplitude: f32, frequency: f32, phase_shift: f32, r: u8, g: u8, b: u8) {
        let mut x_prev = 0;
        let mut y_prev = (self.height as f32 / 2.0 + (amplitude * (frequency * 0.0 + phase_shift)).tan()) as usize;

        for i in 1..self.width {
            let y = (self.height as f32 / 2.0 + (amplitude * (frequency * i as f32 + phase_shift)).tan()) as usize;
            self.draw_line(x_prev, y_prev, i, y, r, g, b);
            x_prev = i;
            y_prev = y;
        }
    }

    // Method to display a PNG image on the LED matrix
    pub fn display_png_image(&mut self, file_path: &str) -> Result<(), image::ImageError> {
        // Load the PNG image
        let img = image::open(file_path)?;

        // Resize the image to fit the LED matrix dimensions
        let resized_img = img.resize_exact(self.width as u32, self.height as u32, image::imageops::FilterType::Nearest);

        // Iterate through each pixel of the resized image and set corresponding LED color
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = resized_img.get_pixel(x as u32, y as u32);
                self.set_led(x, y, pixel[0], pixel[1], pixel[2]);
            }
        }

        Ok(())
    }

    // Method to display a JPEG image on the LED matrix
    pub fn display_jpeg_image(&mut self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Read JPEG file bytes
        let jpeg_file = std::fs::read(file_path)?;

        // Decode JPEG data
        let mut decoder = Decoder::new(&jpeg_file[..]);
        let image_info = decoder.decode().unwrap().0;
        let pixels = decoder.decode_scanline().unwrap();

        // Create an empty buffer to store RGB pixel values
        let mut buffer = vec![0u8; self.width * self.height * 3];

        // Convert YCbCr to RGB
        for (i, pixel) in pixels.chunks(3).enumerate() {
            let r = (1.0 * pixel[0] as f32 + 0.0 * pixel[1] as f32 + 1.402 * pixel[2] as f32) as u8;
            let g = (1.0 * pixel[0] as f32 - 0.344136 * pixel[1] as f32 - 0.714136 * pixel[2] as f32) as u8;
            let b = (1.0 * pixel[0] as f32 + 1.772 * pixel[1] as f32 + 0.0 * pixel[2] as f32) as u8;

            let x = i % image_info.width as usize;
            let y = i / image_info.width as usize;

            let index = ((y * self.width) + x) * 3;

            if index + 2 < buffer.len() {
                buffer[index] = r;
                buffer[index + 1] = g;
                buffer[index + 2] = b;
            }
        }

        // Copy the pixel data into the LED buffer
        self.buffer = buffer;

        Ok(())
    }

}
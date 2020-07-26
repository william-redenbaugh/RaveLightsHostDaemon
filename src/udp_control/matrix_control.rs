// The protobuffer crage that we are using
extern crate quick_protobuf;

// Protobuffer writing module
use quick_protobuf::Writer;
// Standard UDP socket library
use std::net::UdpSocket;

// Crate that contains our message data.
use crate::messagedata;

// Refcel and Rc stuff so we can have multiple pointers to the same stuff
use std::cell::{Cell, RefCell};
use std::rc::Rc;

// Matrix Controller Object for a variable sized panel
pub struct MatrixControl {
    pub socket: Rc<RefCell<UdpSocket>>,
    pub address_port: String,
    pub out_arr: Box<[u8]>,
    pub x_len: u8,
    pub y_len: u8,
}

pub fn new_matrix_control(
    port_ref: Rc<RefCell<UdpSocket>>,
    x_len: u8,
    y_len: u8,
    address_port: String,
) -> MatrixControl {
    // Generates the array that we will save out matrix data in.
    // On the heap, then ownership will be passed to MatrixController.
    let matrix_arr: Vec<u8> = vec![0; (x_len * y_len * 3 + 200) as usize];
    let matrix_arr_cnv = matrix_arr.into_boxed_slice();

    // Creates matrix control object to be returned as the function
    let matrix_control = MatrixControl {
        socket: port_ref,
        address_port: address_port,
        out_arr: matrix_arr_cnv,
        x_len: x_len,
        y_len: y_len,
    };

    return matrix_control;
}

// Implementation for our matrix control module.
impl MatrixControl {
    // Since we are going to be modifying values to a class, this is how we do it!
    pub fn begin(&mut self) {
        // Provides messagedata fields.
        let val = messagedata::MessageData {
            message_size: (self.x_len as u32 * self.y_len as u32 * 3) as u32,
            message_type: messagedata::mod_MessageData::MessageType::MATRIX_DATA,
            return_message: false,
        };

        // Scopes out the protobuff messaging so we save memory
        {
            let mut out = Vec::new();
            let mut writer = Writer::new(&mut out);
            writer
                .write_message(&val)
                .expect("Message couldn't write properly");

            // Generally speaking the first
            // Byte indicates the size of the array.
            // But we don't require it for our purposes.
            out.remove(0);

            // Converts into a boxed pointer.
            let msg_fill = out.into_boxed_slice();

            // Fills in the message data that will
            // Indiciate what type of message this is!
            for x in 0..(msg_fill.len()) {
                self.out_arr[x] = msg_fill[x];
            }
        }

        // Sets all values to zero, and pushes off the udp send command
        self.update();
    }

    // Allows us to set our LEDs to a particular value
    pub fn set_led(&mut self, _x: u8, _y: u8, _r: u8, _g: u8, _b: u8) {
        if (_x >= self.x_len) & (_y >= self.y_len) {
            return;
        }

        let mut spot: usize = _y as usize * self.x_len as usize + _x as usize * 3 + 16;

        // Sets our out array spots
        self.out_arr[spot] = _r;
        spot = spot + 1;
        self.out_arr[spot] = _g;
        spot = spot + 1;
        self.out_arr[spot] = _b;
    }

    pub fn update(&self) {
        self.socket
            .borrow_mut()
            .send_to(&self.out_arr, &self.address_port)
            .expect("couldn't send data");
    }
}

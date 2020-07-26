// Protobuffer message create
extern crate quick_protobuf;
use quick_protobuf::Writer;

// Serial crate
extern crate serial;
use serial::prelude::*;
use std::io::prelude::*;

// Internal Protobuffer Crate for
// our own messagess
use crate::messagedata;

// Refcel and Rc stuff so we can have multiple pointers to the same stuff
// Will be used for sharing a single serial object across multiple "objects", that will be found
//
use std::cell::{Cell, RefCell};
use std::rc::Rc;

pub struct StripControl {
    // Out Array For Dealing with Serial TTY Port Stuff
    pub out_arr: Box<[u8]>,
    // Length of array
    pub len: u32,
    // Serial port passover
    pub serial_port: Rc<RefCell<serial::unix::TTYPort>>,
}

// Setting up the Serial Strip stuff.
impl StripControl {
    // Setup the serial interface for the strip control
    pub fn begin_strip(&mut self) {
        // Provides messagedata fields.
        let val = messagedata::MessageData {
            message_size: self.len * 3 as u32,
            message_type: messagedata::mod_MessageData::MessageType::LED_STRIP_DATA,
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

        // Update strip
        self.update_strip();
    }

    pub fn set_led(&mut self, _led: u32, _r: u8, _g: u8, _b: u8) {
        // Spot in our array.
        let mut spot: usize = (_led * 3) as usize + 16;

        // Sets our out array spots
        self.out_arr[spot] = _r;
        spot = spot + 1;
        self.out_arr[spot] = _g;
        spot = spot + 1;
        self.out_arr[spot] = _b;
    }

    pub fn update_strip(&mut self) {
        let _result = self.serial_port.borrow_mut().write(&self.out_arr);
    }
}

pub fn new_serial_strip(
    num_leds: u32,
    port_ref: Rc<RefCell<serial::unix::TTYPort>>,
) -> StripControl {
    let arr_size = (16 + 3 * num_leds) as usize;

    // Generates the array that we will save out matrix data in.
    // On the heap, then ownership will be passed to MatrixController.
    let matrix_arr: Vec<u8> = vec![0; arr_size];
    let matrix_arr_cnv = matrix_arr.into_boxed_slice();

    let strip_control = StripControl {
        out_arr: matrix_arr_cnv,
        len: num_leds,
        serial_port: port_ref,
    };

    return strip_control;
}

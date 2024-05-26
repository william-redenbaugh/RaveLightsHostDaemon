#include "usb_driver.h"
#include "global_includes.h"

#include <FastLED.h>

// How many leds in your strip?
#define NUM_LEDS 800

// For led chips like WS2812, which have a data line, ground, and power, you just
// need to define DATA_PIN.  For led chipsets that are SPI based (four wires - data, clock,
// ground, and power), like the LPD8806 define both DATA_PIN and CLOCK_PIN
// Clock pin only needed for SPI based chipsets when not using hardware SPI
#define DATA_PIN 3
#define CLOCK_PIN 13

// Define the array of leds
uint8_t input_buffer[2048];
int current_pixel_index = 0;
CRGB leds[NUM_LEDS];

void usb_driver_init(void *parameters)
{
    SerialUSB.begin();
    FastLED.addLeds<NEOPIXEL, DATA_PIN>(leds, NUM_LEDS);  // GRB ordering is assumed
    
}

static inline void interpret_led_message(uint8_t mode, int num_leds){
        switch(mode){

            case 0:{
                current_pixel_index = 0;
                for(int n = 0; n < num_leds; n++){
                    int index = num_leds * 3;
                    leds[n].r = input_buffer[index];
                    leds[n].g = input_buffer[index + 1];
                    leds[n].b = input_buffer[index + 2];
                }
                current_pixel_index += num_leds;
            }
            break;

            case 1:{
                for(int n = 0; n < num_leds; n++){
                    int index = num_leds * 3;
                    leds[n + current_pixel_index].r = input_buffer[index];
                    leds[n + current_pixel_index].g = input_buffer[index + 1];
                    leds[n + current_pixel_index].b = input_buffer[index + 2];
                }
                current_pixel_index += num_leds;
            }
            break;

            case 2:{
                for(int n = 0; n < num_leds; n++){
                    int index = num_leds * 3;
                    leds[n + current_pixel_index].r = input_buffer[index];
                    leds[n + current_pixel_index].g = input_buffer[index + 1];
                    leds[n + current_pixel_index].b = input_buffer[index + 2];
                }
                current_pixel_index += num_leds;
                FastLED.show();
            }
            break;

            default:
            break;
        }
}

void usb_driver_thread(void *parameters)
{
    for(;;){

        // Block until there's enough bytes in queue
        while(SerialUSB.available() < 3){
            os_thread_sleep_ms(5);
        }

        uint8_t mode =  SerialUSB.read();
        uint8_t flags = SerialUSB.read();
        uint8_t num_leds= SerialUSB.read();

        int num_leds_bytes = num_leds * 3;
        int total_led_bytes_so_far = 0;
        
        while(total_led_bytes_so_far < num_leds_bytes){
            int availableBytes = SerialUSB.available();
            // Copy whatever is available
            SerialUSB.readBytes((char*)&input_buffer[total_led_bytes_so_far], availableBytes);
            os_thread_sleep_ms(1);
            total_led_bytes_so_far += availableBytes;
        } 

        interpret_led_message(mode, num_leds);
    }
}
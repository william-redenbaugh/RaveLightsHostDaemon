#include "usb_driver.h"
#include "global_includes.h"
#include <FastLED.h>
#include "project_defs.h"
#include "rgbMatrix.h"

enum{
    START_LED_STRIP_TRANSAC, 
    CONTINUE_LED_STRIP_TRANSAC,
    END_LED_STRIP_TRANSAC,
    INIT_LED_STRIP   
};

uint8_t input_buffer[2048];
int current_pixel_index = 0;

#ifdef LED_STRIP_MODE

#define DATA_PIN 3
#define CLOCK_PIN 13

// Define the array of leds
uint32_t num_leds = 128;
int current_pixel_index = 0;
CRGB *leds;

static inline void start_led_transaction(void){
    current_pixel_index = 0;
    for(int n = 0; n < num_leds; n++){
        int index = num_leds * 3;
        leds[n].r = input_buffer[index];
        leds[n].g = input_buffer[index + 1];
        leds[n].b = input_buffer[index + 2];
    }
    current_pixel_index += num_leds;
}

static inline void continue_led_transaction(void){
    for(int n = 0; n < num_leds; n++){
        int index = num_leds * 3;
        leds[n + current_pixel_index].r = input_buffer[index];
        leds[n + current_pixel_index].g = input_buffer[index + 1];
        leds[n + current_pixel_index].b = input_buffer[index + 2];
    }
    current_pixel_index += num_leds;
}

static inline void complete_led_transaction(void){
    for(int n = 0; n < num_leds; n++){
        int index = num_leds * 3;
        leds[n + current_pixel_index].r = input_buffer[index];
        leds[n + current_pixel_index].g = input_buffer[index + 1];
        leds[n + current_pixel_index].b = input_buffer[index + 2];    }
    current_pixel_index += num_leds;
    FastLED.show();
}

static inline void initialize_led(void){
    leds = (CRGB*)malloc(sizeof(CRGB) * num_leds);
    FastLED.addLeds<NEOPIXEL, DATA_PIN>(leds, num_leds);  // GRB ordering is assume
}

#endif

#ifdef MATRIX_MODE

RGBMatrixSLED1734 matrix;
int x = 0;
int y = 0;

static inline void start_led_transaction(void){
    int curr_x;
    int curr_y;
    for(curr_x = x; x < 8; curr_x++){
        for(curr_y = y; y < 8; y++){
            int input_index = current_pixel_index * 3;
            current_pixel_index++;
            uint8_t index[] = {(uint8_t)curr_x, (uint8_t)curr_y};
            matrix.draw_point(index, 
                input_buffer[input_index], 
                input_buffer[input_index + 1], 
                input_buffer[input_index + 2]
            );
        }
    }    
    x = curr_x;
    y = curr_y;
}

static inline void continue_led_transaction(void){
    int curr_x;
    int curr_y;

    for(curr_x = x; x < 8; curr_x++){
        for(curr_y = y; y < 8; y++){
            int input_index = current_pixel_index * 3;
            current_pixel_index++;
            uint8_t index[] = {(uint8_t)curr_x, (uint8_t)curr_y};
            matrix.draw_point(index, 
                input_buffer[input_index], 
                input_buffer[input_index + 1], 
                input_buffer[input_index + 2]
            );
        }
    }    
    x = curr_x;
    y = curr_y;
}

static inline void complete_led_transaction(void){
    int curr_x;
    int curr_y;

    for(curr_x = x; x < 8; curr_x++){
        for(curr_y = y; y < 8; y++){
            int input_index = current_pixel_index * 3;
            current_pixel_index++;
            uint8_t index[] = {(uint8_t)curr_x, (uint8_t)curr_y};
            matrix.draw_point(index, 
                input_buffer[input_index], 
                input_buffer[input_index + 1], 
                input_buffer[input_index + 2]
            );
        }
    }    
    x = curr_x;
    y = curr_y;
}

static inline void initialize_led(void){
    matrix.RGBMatrixInit();
}

#endif
static inline void interpret_message(uint8_t mode, int num_leds){
    switch(mode){
        case START_LED_STRIP_TRANSAC:
            start_led_transaction();
        break;
        case CONTINUE_LED_STRIP_TRANSAC:
            continue_led_transaction();
        break;
        case END_LED_STRIP_TRANSAC:
            complete_led_transaction();
        break;
        case INIT_LED_STRIP:
            initialize_led();
        break;
        default:
        break;
    }
}

void usb_driver_init(void *parameters)
{
    SerialUSB.begin();
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

        interpret_message(mode, num_leds);
    }
}
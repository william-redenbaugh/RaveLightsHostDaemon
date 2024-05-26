#ifndef _PROJECT_DEFS_H
#define _PROJECT_DEFS_H
#include "Arduino.h"

#define POWR_TWO(x) (1 << (x))
#define NUM_SAMPLES_POW 8
#define MIC_SAMPLES POWR_TWO(NUM_SAMPLES_POW)
#define MIC_SAMPLE_RATE 24000

#define INTENSITY_DIVIDER_DEFAULT (3000000)
#define INTENSITY_DIVIDER_MAX (12000000)
#define INTENSITY_DIVIDER_MIN (1000000)

// most microphones will probably default to left channel but you may need to tie the L/R pin low
#define I2S_MIC_CHANNEL I2S_CHANNEL_FMT_ONLY_LEFT

#ifdef CONFIG_IDF_TARGET_ESP32S3
// either wire your microphone to the same pins or change these to match your wiring
#define I2S_MIC_SERIAL_CLOCK GPIO_NUM_35
#define I2S_MIC_LEFT_RIGHT_CLOCK GPIO_NUM_36
#define I2S_MIC_SERIAL_DATA GPIO_NUM_37
#define I2S_MIC_SERIAL_DATA_CS GPIO_NUM_7

#elif CONFIG_IDF_TARGET_ESP32C3
#define I2S_MIC_SERIAL_CLOCK GPIO_NUM_20
#define I2S_MIC_LEFT_RIGHT_CLOCK GPIO_NUM_19
#define I2S_MIC_SERIAL_DATA GPIO_NUM_18
#define I2S_MIC_SERIAL_DATA_CS GPIO_NUM_7

#endif

#endif
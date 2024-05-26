#include "usb_driver.h"
#include "global_includes.h"

void usb_driver_init(void *parameters)
{
    SerialUSB.begin();
}

void usb_driver_thread(void *parameters)
{
    for(;;){

    }
}
#ifndef _THREADS_LIST_H
#define _THREADS_LIST_H
#include "global_includes.h"
#include "usb_driver.h"
const task_init_descriptor_t INIT_THREAD_LIST[] =
    {
        {usb_driver_thread, usb_driver_init, "USB Driver module", 4096, NULL, 0, NULL},
};
extern int NUM_THREADS;
#endif
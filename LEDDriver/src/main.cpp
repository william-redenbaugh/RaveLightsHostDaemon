#include <Arduino.h>
#include "global_includes.h"
#include "OS/OSThreadKernel.h"
void setup()
{
    threads_init();
    threads_list_init();
}

void loop()
{
    os_thread_delay_s(20);
}
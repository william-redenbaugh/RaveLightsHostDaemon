#ifndef _THREADS_LIST_H
#define _THREADS_LIST_H

#include "matrix_thread.h"
#include "../src/CSAL_SHARED/event_management.h"
#include "../src/CHAL_SHARED/csal_ipc_thread.h"

#define INIT_THREAD_LIST                                                                                                \
    (task_init_descriptor_t[]){                                                                                         \
        {event_management_thread, event_management_init, "Event Management Thread", 2048, NULL, 0, NULL},               \
        {ipc_consume_thread, ipc_consume_thread_init, "IPC consume thread", 4096, NULL, 0, NULL},                       \
        {ipc_publish_thread, ipc_publish_init, "IPC consume thread", 4096, NULL, 0, NULL},                              \
    }                                                                                                                   \

extern int NUM_THREADS;
#endif
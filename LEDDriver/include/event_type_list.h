#ifndef _EVENT_TYPE_LIST_H
#define _EVENT_TYPE_LIST_H

typedef enum
{
    EVENT_NONE = 0,
    EVENT_LED_UPDATE,
    EVENT_END,
} event_type_t;

typedef struct event_data
{
    event_type_t event_id;
    void *data_ptr;
} event_data_t;

#define EVENT_TYPE_EVENT_END EVENT_END

#endif
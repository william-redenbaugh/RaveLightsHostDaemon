#ifndef _PROJECT_CONF_H
#define _PROJECT_CONF_H

#include "global_includes.h"

/**
 * @brief Largest message size over UART
*/
#define IPC_MSG_MAX_SIZE 64000

/**
 * @brief The default interface we are using
*/
#define DEFAULT_INTERFACE_TYPE IPC_TYPE_UDP
#endif
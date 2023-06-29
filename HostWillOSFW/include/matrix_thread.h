#ifndef _MATRIX_THREAD_H
#define _MATRIX_THREAD_H

/**
 * @brief Thread that will handle all of our matrix stuff
*/
void matrix_thread(void *params);

/**
 * @brief Initialization for our led matrix
*/
void matrix_thread_init(void *params);

#endif
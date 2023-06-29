#include "stdio.h"
#include "global_includes.h"
#include "pthread.h"
#include "unistd.h"

int main(void){
    threads_list_init();
    for(;;){
        sleep(320);
    }
}
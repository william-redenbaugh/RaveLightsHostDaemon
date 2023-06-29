cmake_minimum_required(VERSION 3.17)
PROJECT(WillOSHostRaveLights)

file(GLOB_RECURSE SRCS src/*.cpp)
file(GLOB_RECURSE HDRS src/*.h)

file(GLOB_RECURSE INCL_HDRS include/*.h)

ADD_EXECUTABLE(WillOSHostRaveLights  
    ${INCL_HDRS} ${SRCS_HAL} ${HDRS_HAL}
    ${SRCS} ${HDRS})

target_include_directories(WillOSHostRaveLights
    PRIVATE
    include/)

set(THREADS_PREFER_PTHREAD_FLAG ON)
find_package(Threads REQUIRED)
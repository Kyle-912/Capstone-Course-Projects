cmake_minimum_required(VERSION 3.12)

include(pico_sdk_import.cmake)

project(level C CXX ASM)
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
pico_sdk_init()

add_executable(level
    src/level.cpp
)

pico_generate_pio_header(level ${CMAKE_CURRENT_LIST_DIR}/libraries/neopixel.pio OUTPUT_DIR ${CMAKE_CURRENT_LIST_DIR}/libraries/generated)

target_sources(level PRIVATE libraries/neomatrix.cpp libraries/neomatrix.h)
target_sources(level PRIVATE libraries/LIS3DH.cpp libraries/LIS3DH.h)
target_sources(level PRIVATE src/level.cpp)

target_link_libraries(level PRIVATE pico_stdlib pico_binary_info hardware_i2c hardware_pio hardware_clocks)

pico_enable_stdio_usb(level 1)

pico_add_extra_outputs(level)
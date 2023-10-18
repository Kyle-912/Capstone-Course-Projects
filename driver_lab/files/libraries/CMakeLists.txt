add_executable(level
        /home/wsl/CEN3907/driver_lab/files/src/level.cpp
        )

pico_enable_stdio_usb(level 1)

# pull in common dependencies and additional i2c hardware support
target_link_libraries(level pico_stdlib hardware_i2c)

# create map/bin/hex file etc.
pico_add_extra_outputs(level)

#------------------------------------------------------------------

# Add your executable for NeoMatrix (neomatrix.cpp)
add_executable(neomatrix)

# Generate the header file for your neopixel.pio into the source tree
file(MAKE_DIRECTORY ${CMAKE_CURRENT_LIST_DIR}/generated)
pico_generate_pio_header(neomatrix ${CMAKE_CURRENT_LIST_DIR}/neopixel.pio OUTPUT_DIR ${CMAKE_CURRENT_LIST_DIR}/generated)

# Set the source files for your neomatrix
target_sources(neomatrix PRIVATE neomatrix.cpp)

# Link the necessary libraries (modify as needed)
target_link_libraries(neomatrix PRIVATE pico_stdlib hardware_pio)

# Add extra build outputs if required
pico_add_extra_outputs(neomatrix)

add_subdirectory(libraries)

# add_executable(pio_ws2812_parallel)

# pico_generate_pio_header(pio_ws2812_parallel ${CMAKE_CURRENT_LIST_DIR}/ws2812.pio OUTPUT_DIR ${CMAKE_CURRENT_LIST_DIR}/generated)

# target_sources(pio_ws2812_parallel PRIVATE ws2812_parallel.c)

# target_compile_definitions(pio_ws2812_parallel PRIVATE
#         PIN_DBG1=3)

# target_link_libraries(pio_ws2812_parallel PRIVATE pico_stdlib hardware_pio hardware_dma)
# pico_add_extra_outputs(pio_ws2812_parallel)

# # add url via pico_set_program_url
# example_auto_set_url(pio_ws2812_parallel)

# # Additionally generate python and hex pioasm outputs for inclusion in the RP2040 datasheet
# add_custom_target(pio_ws2812_datasheet DEPENDS ${CMAKE_CURRENT_LIST_DIR}/generated/ws2812.py)
# add_custom_command(OUTPUT ${CMAKE_CURRENT_LIST_DIR}/generated/ws2812.py
#         DEPENDS ${CMAKE_CURRENT_LIST_DIR}/ws2812.pio
#         COMMAND Pioasm -o python ${CMAKE_CURRENT_LIST_DIR}/ws2812.pio ${CMAKE_CURRENT_LIST_DIR}/generated/ws2812.py
#         VERBATIM)
# add_dependencies(pio_ws2812 pio_ws2812_datasheet)
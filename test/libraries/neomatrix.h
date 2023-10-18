#pragma once

#include <cstdint>

#include <stdlib.h>
#include <vector>
#include "pico/stdlib.h"
#include "hardware/pio.h"
#include "hardware/clocks.h"
#include "neopixel.pio.h"

class NeoMatrix
{
public:
    std::vector<std::vector<uint32_t>> matrix;

    // Class constructor, used to instantiate an object
    NeoMatrix(uint8_t width, uint8_t height);

    // Initialize the object, returning true on success or false on failure
    bool init();

    // Set the pixel at row row and column col (zero indexed) to color
    void set_pixel(uint8_t row, uint8_t col, uint32_t color);

    // Set all elements of the pixel buffer to 0x00
    void clear_pixels();

    // Write the pixel buffer to the NeoMatrix
    void write();
};

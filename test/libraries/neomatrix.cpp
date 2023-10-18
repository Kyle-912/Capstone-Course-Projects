#include "neomatrix.h"

// Class constructor, used to instantiate an object
NeoMatrix::NeoMatrix(uint8_t width, uint8_t height)
{
    matrix.resize(width, std::vector<uint32_t>(height, 0));
}

// Initialize the object, returning true on success or false on failure
bool NeoMatrix::init()
{
    // allow neo matrix to get power pin
    const uint LED_PIN = 10;
    gpio_init(LED_PIN);
    gpio_set_dir(LED_PIN, 1);
    gpio_put(LED_PIN, 1);

    PIO pio = pio0;
    int sm = 0;
    uint offset = pio_add_program(pio, &ws2812_program);

    ws2812_program_init(pio, sm, offset, 7, 800000, false);

    return true; // Idk how this would report a fail
}

// Set the pixel at row row and column col (zero indexed) to color
void NeoMatrix::set_pixel(uint8_t row, uint8_t col, uint32_t color)
{
    matrix[row][col] = color;
}

// Set all elements of the pixel buffer to 0
void NeoMatrix::clear_pixels()
{
    matrix.assign(matrix.size(), std::vector<uint32_t>(matrix[0].size(), 0));
}

// Write the pixel buffer to the NeoMatrix
void NeoMatrix::write()
{
    for (int row = 0; row < matrix.size(); row++)
    {
        for (int col = 0; col < matrix[0].size(); col++)
        {
            pio_sm_put_blocking(pio0, 0, matrix[row][col] << 8u);
        }
    }
}
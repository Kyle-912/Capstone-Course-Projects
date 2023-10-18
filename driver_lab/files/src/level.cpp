#include "../libraries/LIS3DH.h"
#include "../libraries/neomatrix.h"
#include <cmath>

#include <cstdio>

int main()
{
    LIS3DH accel;
    NeoMatrix matrix(8, 8);

    accel.init();
    matrix.init();

    while (1)
    {
        matrix.clear_pixels();

        accel.update();
        float x = accel.x;
        float y = accel.y;
        float z = accel.z;

        // Clamp values to between -1 and 1
        if (x > 1.0)
        {
            x = 1.0;
        }
        else if (x < -1.0)
        {
            x = -1.0;
        }
        if (y > 1.0)
        {
            y = 1.0;
        }
        else if (y < -1.0)
        {
            y = -1.0;
        }

        if (fabs(x) <= 0.1 && fabs(y) <= 0.1)
        {
            matrix.set_pixel(3, 3, 0x00FF0000);
            matrix.set_pixel(3, 4, 0x00FF0000);
            matrix.set_pixel(4, 4, 0x00FF0000);
            matrix.set_pixel(4, 3, 0x00FF0000);
        }
        else
        {
            int row = static_cast<int>((y + 1) * 3.5); // Map x to the LED matrix width
            int col = static_cast<int>((x + 1) * 3.5); // Map y to the LED matrix height

            // Ensure that the mapped pixel position is within the LED matrix boundaries
            row = std::max(0, std::min(7, row));
            col = std::max(0, std::min(7, col));

            // Set the determined pixel to red
            matrix.set_pixel(row, col, 0x0000FF00);
        }

        matrix.write();

        // TODO: Remove debug prints
        printf("X acceleration: %.3fg\n", accel.x);
        printf("Y acceleration: %.3fg\n", accel.y);
        printf("Z acceleration: %.3fg\n", accel.z);
        sleep_ms(50);
        // Clear terminal
        printf("\033[1;1H\033[2J");
    }
}

// red: 0x0000FF00
// green: 0x00FF0000
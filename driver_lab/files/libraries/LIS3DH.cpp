#include "LIS3DH.h"

// Constructor
LIS3DH::LIS3DH()
{
    // Class members representing the three acceleration values in units of g (Earth’s gravitational acceleration)
    x = 0.0;
    y = 0.0;
    z = 0.0;
}

// Initializes the accelerometer, returning true on success or false on failure
bool LIS3DH::init()
{
    stdio_init_all();

    i2c_init(i2c1, 400 * 1000);
    gpio_set_function(2, GPIO_FUNC_I2C);
    gpio_set_function(3, GPIO_FUNC_I2C);
    gpio_pull_up(2);
    gpio_pull_up(3);
    // Make the I2C pins available to picotool
    bi_decl(bi_2pins_with_func(2, 3, GPIO_FUNC_I2C));

    uint8_t buf[2];

    // Turn normal mode and 1.344kHz data rate on
    buf[0] = CTRL_REG_1;
    buf[1] = 0x97;
    i2c_write_blocking(i2c1, ADDRESS, buf, 2, false);

    return true; // Idk how this would report a fail
}

// Set a register on the LIS3DH to the given value
void LIS3DH::set_reg(uint8_t reg, uint8_t val)
{
    uint8_t buf[2];
    buf[0] = reg;
    buf[1] = val;
    i2c_write_blocking(i2c1, ADDRESS, buf, 2, false);
}

// Read and return the byte at address reg on the accelerometer
uint8_t LIS3DH::read_reg(uint8_t reg)
{
    uint8_t val;
    i2c_write_blocking(i2c1, ADDRESS, &reg, 1, true);
    i2c_read_blocking(i2c1, ADDRESS, &val, 1, false);
    return val;
}

// Updates the class members x, y, and z with the current acceleration values
void LIS3DH::update()
{
    uint8_t reg;
    uint8_t lsb;
    uint8_t msb;
    uint16_t raw_accel;

    reg = 0x28;
    i2c_write_blocking(i2c1, ADDRESS, &reg, 1, true);
    i2c_read_blocking(i2c1, ADDRESS, &lsb, 1, false);
    reg |= 0x01;
    i2c_write_blocking(i2c1, ADDRESS, &reg, 1, true);
    i2c_read_blocking(i2c1, ADDRESS, &msb, 1, false);
    raw_accel = (msb << 8) | lsb;
    float scaling;
    float sensitivity = 0.004f; // g per unit
    scaling = 64 / sensitivity;
    x = (float)((int16_t)raw_accel) / scaling;

    reg = 0x2A;
    i2c_write_blocking(i2c1, ADDRESS, &reg, 1, true);
    i2c_read_blocking(i2c1, ADDRESS, &lsb, 1, false);
    reg |= 0x01;
    i2c_write_blocking(i2c1, ADDRESS, &reg, 1, true);
    i2c_read_blocking(i2c1, ADDRESS, &msb, 1, false);
    raw_accel = (msb << 8) | lsb;
    scaling = 64 / sensitivity;
    y = (float)((int16_t)raw_accel) / scaling;

    reg = 0x2C;
    i2c_write_blocking(i2c1, ADDRESS, &reg, 1, true);
    i2c_read_blocking(i2c1, ADDRESS, &lsb, 1, false);
    reg |= 0x01;
    i2c_write_blocking(i2c1, ADDRESS, &reg, 1, true);
    i2c_read_blocking(i2c1, ADDRESS, &msb, 1, false);
    raw_accel = (msb << 8) | lsb;
    scaling = 64 / sensitivity;
    z = (float)((int16_t)raw_accel) / scaling;
}
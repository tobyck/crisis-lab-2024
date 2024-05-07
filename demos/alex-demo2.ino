#include <Wire.h>
#include <SparkFun_LPS28DFW_Arduino_Library.h>

// Create a new sensor object
LPS28DFW pressureSensor;

// I2C address selection
uint8_t i2cAddress = LPS28DFW_I2C_ADDRESS_DEFAULT; // 0x5C
//uint8_t i2cAddress = LPS28DFW_I2C_ADDRESS_SECONDARY; // 0x5D

void setup()
{
    // Start serial
    Serial.begin(115200);
    Serial.println("LPS28DFW Example 2 - Filtering");

    // Initialize the I2C library
    Wire.begin();

    // Check if sensor is connected and initialize
    // Address is optional (defaults to 0x5C)
    while(pressureSensor.begin(i2cAddress) != LPS28DFW_OK)
    {
        // Not connected, inform user
        Serial.println("Error: LPS28DFW not connected, check wiring and I2C address!");

        // Wait a bit to see if connection is established
        delay(1000);
    }

    Serial.println("LPS28DFW connected!");

    // Here we configure the sensor to have minimal measurement noise. There are
    // 4 parameters that we can control:
    // 
    // Full scale range - 2 options are available: 1260hPa, and 4000hPa. The
    // sensor has a fixed number of bits for each measurement, so the lower
    // range results in better resolution
    // 
    // Average filter - The sensor takes some number of samples (min 4, max 512)
    // and averages them together. Note that larger numbers of samples results
    // in more current consumption, and a lower maximum ODR. 25Hz ODR is valid
    // for all cases, see datasheet for more info
    // 
    // Low-pass filter - This is an optional filter, which is applied after the
    // average filter. The cutoff frequency depends on the ODR, and can be
    // configured as either ODR/4 or ODR/9
    // 
    // Output data rate - Other than the low-pass filter, this has no effect on
    // the measurement noise. Note that the maximum value for this depends on
    // the average filter, 25Hz is valid for all average filter settings. See
    // datasheet for details
    lps28dfw_md_t modeConfig =
    {
        .fs  = LPS28DFW_1260hPa,      // Full scale range
        .odr = LPS28DFW_25Hz,         // Output data rate
        .avg = LPS28DFW_512_AVG,      // Average filter
        .lpf = LPS28DFW_LPF_ODR_DIV_9 // Low-pass filter
    };
    pressureSensor.setModeConfig(&modeConfig);
}

void loop()
{
    // Get measurements from the sensor. This must be called before accessing
    // the pressure data, otherwise it will never update
    pressureSensor.getSensorData();

    // Print temperature and pressure
    Serial.print("Temperature (C): ");
    Serial.print(pressureSensor.data.heat.deg_c);
    Serial.print("\t\t");
    Serial.print("Pressure (hPa): ");
    Serial.println(pressureSensor.data.pressure.hpa);

    // Print at 25Hz
    delay(40);
}
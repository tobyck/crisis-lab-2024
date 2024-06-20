/*
 * Author: Toby Connor-Kebbell & Maxwell Robati
 * Version: 9/06/2024
 * Purpose: Gather data from pressure sensor.
 */

#include "sensor.hpp"

/*
 *	Initialize the sensor.
 */

void init_sensor(LPS28DFW sensor) {
	uint8_t i2c_address = LPS28DFW_I2C_ADDRESS_DEFAULT;

	Serial.begin(115200);
	Wire.begin();

	// If it can't connect, keep trying every second until it does.
	while (sensor.begin(i2c_address) != LPS28DFW_OK) {
		Serial.println("Error: sensor failing to connect.");
		delay(1000);
	}
}

/*
 *	Get pressure data in hPa.
 */

float get_pressure(LPS28DFW sensor) {
	sensor.getSensorData();
	return sensor.data.pressure.hpa;
}

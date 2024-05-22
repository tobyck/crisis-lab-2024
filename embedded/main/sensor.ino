/*
 * Author: Toby Connor-Kebbell & Maxwell Robati
 * Version: 12/05/2024
 * Purpose: Gather data from pressure sensor
 */
#include "sensor.hpp"

void init_sensor(LPS28DFW sensor) {
	uint8_t i2c_address = LPS28DFW_I2C_ADDRESS_DEFAULT;

	Serial.begin(115200);
	Wire.begin();

	// If it can't connect, keep trying until it does
	while (sensor.begin(i2c_address) != LPS28DFW_OK) {
		Serial.println("Error: sensor failing to connect.");
		delay(1000);
	}
}

float get_pressure(LPS28DFW sensor) {
	sensor.getSensorData();
	return sensor.data.pressure.hpa;
}
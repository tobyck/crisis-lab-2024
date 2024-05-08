#include "sensor.h"

void init_sensor(LPS28DFW sensor) {
	uint8_t i2c_address = LPS28DFW_I2C_ADDRESS_DEFAULT;

	Serial.begin(115200);
	Wire.begin();

	// if it can't connect, keep erroring until it does
	while (sensor.begin(i2c_address) != LPS28DFW_OK) {
		Serial.println("Error: sensor failing to connect.");
		delay(1000);
	}
}

float get_pressure(LPS28DFW sensor) {
	sensor.getSensorData();
	return sensor.data.pressure.hpa;
}

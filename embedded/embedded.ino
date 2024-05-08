#include "sensor.h"
// #include <ArduinoWebsockets.h>
#include <ESP8266WiFi.h>

LPS28DFW sensor;

void setup() {
	init_sensor(sensor);
}

void loop() {
	Serial.print("Pressure: ");
	Serial.println(get_pressure(sensor));
	delay(500);
}

/*
 * Author: Toby Connor-Kebbell & Maxwell Robati
 * Version: 12/05/2024
 * Purpose: Collect sensor data and transmit it to the WiFi card
 */

#include "sensor.hpp"

// Virtual Serial to output to WiFi card, Recieve pin 2, Transmit pin 3.
SoftwareSerial outputSerial = SoftwareSerial(RXPin, TXPin);
LPS28DFW sensor;

void setup() {
	init_sensor(sensor);
  outputSerial.begin(9600);
}

void loop() {
  float pressure = get_pressure(sensor);
	Serial.print("Pressure: ");
	Serial.println(pressure);
  // Send pressure data to WiFi card
  outputSerial.println(pressure);
	delay(83);
}

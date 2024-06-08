/*
 * Author: Toby Connor-Kebbell & Maxwell Robati
 * Version: 8/06/2024
 * Purpose: Collect sensor data and transmit it to the WiFi card
 */

#include "sensor.hpp"

// Virtual Serial to output to WiFi card, Recieve pin 2, Transmit pin 3.
SoftwareSerial outputSerial = SoftwareSerial(RXPin, TXPin);
LPS28DFW sensor;

void setup() {
  /*
   * Init everything
   */
  pinMode(CAPin, INPUT);
  pinMode(CWPin, INPUT);
	init_sensor(sensor);
  Serial.begin(9600);
  outputSerial.begin(9600);
}

void loop() {
  /*
   * Air Pressure Calibration
   */
  // When the button is pressed
  if(digitalRead(CAPin) == 1) {
    calibratingAir = true;
  } else {
    if(calibratingAir == true) {
      // When the button is released
      calibratingAir = false;
      outputSerial.println("C AIR");
      Serial.println("C AIR");
      return;
    }
  }
  /*
   * Water Pressure Calibration
   */
  // When the button is pressed
  if(digitalRead(CWPin) == 1) {
    calibratingWater = true;
  } else {
    if(calibratingWater == true) {
      // When the button is released
      calibratingWater = false;
      outputSerial.println("C WATER");
      Serial.println("C WATER");
      return;
    }
  }

  /*
   * Pressure logging
   */
  float pressure = get_pressure(sensor);
	Serial.print("Pressure: ");
	Serial.println(pressure);
  // Send pressure data to WiFi card
  outputSerial.println(pressure);
	delay(40);
}
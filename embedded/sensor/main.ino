/*
 * Author: Maxwell Robati
 * Version: 9/06/2024
 * Purpose: Collect sensor data and transmit it to the WiFi card.
 */

#include "sensor.hpp"

SoftwareSerial wifiCardSerial = SoftwareSerial(WifiReadPin, WifiWritePin);
LPS28DFW sensor;

// NOTE: anything printed to Serial is for debugging purposes

void setup() {
    pinMode(AirCalibrationPin, INPUT);
    pinMode(WaterCalibrationPin, INPUT);

    init_sensor(sensor);

    Serial.begin(9600);
    wifiCardSerial.begin(9600);
}

void loop() {
    if (digitalRead(AirCalibrationPin) == 1) {
        airCalibrationPressed = true;
    } else if (airCalibrationPressed) {
        airCalibrationPressed = false;

        wifiCardSerial.println("C AIR");
        Serial.println("C AIR");
    }

    if (digitalRead(WaterCalibrationPin) == 1) {
        waterCalibrationPressed = true;
    } else if (waterCalibrationPressed) {
        waterCalibrationPressed = false;

        wifiCardSerial.println("C WATER");
        Serial.println("C WATER");
    }

    float pressure = get_pressure(sensor);
    Serial.print("Pressure: ");
    Serial.println(pressure);

    wifiCardSerial.println(pressure);

    delay(40);
}

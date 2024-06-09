/*
 * Author: Toby Connor-Kebbell & Maxwell Robati
 * Version: 9/06/2024
 * Purpose: Collect sensor data and transmit it to the WiFi card.
 */

#include "sensor.hpp"

// Virtual Serial to output to WiFi card, Recieve pin 2, Transmit pin 3.
SoftwareSerial outputSerial = SoftwareSerial(RXPin, TXPin);
LPS28DFW sensor;

/*
 *	Init.
 */

void setup() {
        pinMode(CAPin, INPUT);
        pinMode(CWPin, INPUT);

        init_sensor(sensor);

        Serial.begin(9600);
        outputSerial.begin(9600);
}

/*
 *	Main loop.
 */

void loop() {

        /*
         * Air Pressure Calibration
         */

        // When the button is pressed, set its state to pressed.
        if(digitalRead(CAPin) == 1) {
                calibratingAir = true;
        } else {        // If the button isn't currently pressed
                // Check if the button's state is pressed, otherwise ignore as the button is idle.
                if(calibratingAir == true) {
                        // Set the button state to idle.
                        calibratingAir = false;

                        // Transmit message to broker.
                        outputSerial.println("C AIR");
                        Serial.println("C AIR");                        // For debugging
                        return;
                }
        }

        /*
         * Water Pressure Calibration
         */

        // When the button is pressed, set its state to pressed.
        if(digitalRead(CWPin) == 1) {
                calibratingWater = true;
        } else {        // If the button isn't currently pressed
                // Check if the button's state is pressed, otherwise ignore as the button is idle.
                if(calibratingWater == true) {
                        // Set the button state to idle.
                        calibratingWater = false;

                        // Transmit message to broker.
                        outputSerial.println("C WATER");
                        Serial.println("C WATER");                      // For debugging.
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

        delay(40);              // Run at 25Hz.
}

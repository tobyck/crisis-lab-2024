/*
 * Author:  Alex Berry
 * Version: 11/06/2024
 * Purpose: Physical alert system.
 */

#include "alert-arduino.hpp"

/*
 * Init.
 */

void setup() {
	// Servos are attached to pins 8 and 9
	servo1.attach(9);
	servo2.attach(8);

	// Set up Arduino pins
	pinMode(5, OUTPUT);
	pinMode(6, OUTPUT);
	pinMode(7, OUTPUT);

	// Begin Serial and LED
	pixels.begin();
	Serial.begin(9600);
}

/*
 * Main loop.
 */

void loop() {
	// Program Global time
	globalTime = millis();

	if (Serial.available()) {
		data = Serial.readStringUntil('\r');	// Read from python script until carriage return
		// If a trigger was sent
		if (data.startsWith("T")) {
			isTriggering = true;
			alert();

      			Serial.println("High");		// For debugging

			startTime = globalTime;		// Trigger start time
		}
	}

	// If alert has been running for more than 10 seconds
	if(isTriggering == true && ((globalTime - startTime) >= triggerTime)) {
		isTriggering = false;
		Serial.println("Low");	// For debugging
	}
}

/*
 * Alert function.
 */

void alert() {
	// While triggering
	while(isTriggering) {
		// Can use globalTime?
		Serial.print("Global: ");
		Serial.println(globalTime);
		currentTime = millis();
		Serial.print("Current: ");
		Serial.println(currentTime);

		// Speakers
		if (currentTime - prevTimeTones > intervalTones) {
			noTone(currentTonePin);

			if (currentTonePin == 3) {
				currentTonePin = 4;
				currentToneFrequency = 523;
				currentToneDuration = 300;
			} else {
				currentTonePin = 3;
				currentToneFrequency = 440;
				currentToneDuration = 200;
			}

			tone(currentTonePin, currentToneFrequency, currentToneDuration);
			prevTimeTones = currentTime;
		}

		// Servos
		if (currentTime - prevTimeServos > intervalServos) {
			prevTimeServos = currentTime;
			pos += increment;

			if ((pos >= 180) || (pos <= 0)) {
				increment = -increment;
			}

			servo1.write(pos);
			servo2.write(180 - pos);
		}

		// Lights
		if (currentTime - prevTimeLight > intervalLight) {
			prevTimeLight = currentTime;

			for (int i = 0; i < pixels.numPixels(); i++) {
				if ((isWhite + i) % 8 < 4) {
					pixels.setPixelColor(i, pixels.Color(255, 0, 0));
				} else {
					pixels.setPixelColor(i, pixels.Color(255, 255, 255));
				}
			}

			pixels.show();
			isWhite++;
		}
	}; 
}

/*
 * Author:  Alex Berry and Maxwell Robati
 * Version: 16/06/2024
 * Purpose: Physical alert system.
 */

#include "alert-arduino.hpp"

/*
 * Init.
 */

void setup() {
	// Servos is attached to pin 9
	servo1.attach(9);

	// LED is pin 6
	// Speakers are pins 3 and 4

	// Begin Serial and LED
	pixels.begin();
	Serial.begin(9600);
}

/*
 * Main loop.
 */

void loop() {
	// Program's Global time
	globalTime = millis();

	if (Serial.available()) {
		data = Serial.readStringUntil('\r');	// Read from WebSocket Client until carriage return
		// If a trigger was sent
		Serial.println(data);
		if (data.startsWith("T")) {
			isTriggering = true;
      			Serial.println("High");		// For debugging
			alert();
		}
	}
}

/*
 * Alert function.
 */

void alert() {
	while(isTriggering) {
		// If alert has been running for more than 10 seconds, stop running.
		if(((currentTime - globalTime) >= triggerTime)) {
			// Turn all LEDs off
			for(int i = 0; i < pixels.numPixels(); i++) {
				pixels.setPixelColor(i, pixels.Color(0, 0, 0));
			}
			pixels.show();

			// Stop looping
			isTriggering = false;
			Serial.println("Low");	// For debugging
			return;
		}

		currentTime = millis();

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

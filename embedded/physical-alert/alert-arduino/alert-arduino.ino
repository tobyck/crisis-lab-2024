/*
 * Authors: Alex Berry and Maxwell Robati
 * Version: 16/06/2024
 * Purpose: Physical alert system.
 */

#include "alert-arduino.hpp"

void setup() {
	servo1.attach(SERVO_PIN);
	pixels.begin();
	Serial.begin(9600);
}


void loop() {
	alertStartTime = millis();

	if (Serial.available()) {
		data = Serial.readStringUntil('\r');
		Serial.println(data);

		if (data.startsWith("T")) alert();
	}
}


void alert() {
	// note: Because the Arduino is single-threaded, the alert() function will block the loop until it finishes
	// So alertStartTime won't be updated until the function finishes
	while (currentTime - alertStartTime <= alertDuration) {
		currentTime = millis();

		if (currentTime - previousSpeakerTime > speakerInterval) {
			previousSpeakerTime = currentTime;

			noTone(currentTonePin);

			if (currentTonePin == SPEAKER_PIN_1) {
				currentTonePin = SPEAKER_PIN_2;
				tone(currentTonePin, SPEAKER_FREQUENCY_1, SPEAKER_DURATION_1);
			} else {
				currentTonePin = SPEAKER_PIN_1;
				tone(currentTonePin, SPEAKER_FREQUENCY_2, SPEAKER_DURATION_2);
			}
		}

		if (currentTime - previousServoTime > servoInterval) {
			previousServoTime = currentTime;

			servoPosition += servoDirection;
			// We're using a 180 degree servo, so need to rotate the other way when we reach the limits
			if (servoPosition >= 180 || servoPosition <= 0) {
				servoDirection = -servoDirection;
			}

			servo1.write(servoPosition);
		}

		if (currentTime - previousLightTime > lightInterval) {
			previousLightTime = currentTime;

			// This colours every other block of 4 LEDs red
			for (int i = 0; i < pixels.numPixels(); i++) {
				if ((LEDPosition + i) % 8 < 4) {
					pixels.setPixelColor(i, pixels.Color(255, 0, 0));
				} else {
					pixels.setPixelColor(i, pixels.Color(255, 255, 255));
				}
			}

			pixels.show();
			LEDPosition++;
		}
	}

	for(int i = 0; i < pixels.numPixels(); i++) {
		pixels.setPixelColor(i, pixels.Color(0, 0, 0));
	}
	pixels.show();

	return;
}

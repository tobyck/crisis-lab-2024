/*
 * Author: Alex Berry
 * Version: 9/06/2024
 * Purpose: Physical alert system.
 */

#include "alert-arduino.hpp"

/*
 * Init.
 */


void setup() {
	servo1.attach(9);
	servo2.attach(8);
	pinMode(5, OUTPUT);
	pinMode(6, OUTPUT);
	pinMode(7, OUTPUT);
	pixels.begin();
	Serial.begin(9600);
}

/*
 * Main loop.
 */

void loop() {
	current = millis();
    // Serial.println("test");
	if (Serial.available()) {
    Serial.println("test");
		data = Serial.readStringUntil('\r');
		if (data.startsWith("T")) {
			isTriggering = true;
			alertOn();
      Serial.println("high");//for debugging
			start = current;
		}
    
	}


  if (isTriggering == true && ((current - start) >= triggerTime)){
    isTriggering = false;
    Serial.println("low");//for debugging
  }

}

void alertOn() {
	do {
		currentTime = millis();

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

		if (currentTime - prevTimeServos > intervalServos) {
			prevTimeServos = currentTime;
			pos += increment;

			if ((pos >= 180) || (pos <= 0)) {
				increment = -increment;
			}

			servo1.write(pos);
			servo2.write(180 - pos);
		}

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
	} while (isTriggering == true);
}

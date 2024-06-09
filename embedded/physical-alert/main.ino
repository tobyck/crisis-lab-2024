#include <Adafruit_NeoPixel.h>
#include <Adafruit_TiCoServo.h>

// #include <Servo.h>


Adafruit_TiCoServo servo1;
Adafruit_TiCoServo servo2;
Adafruit_NeoPixel pixels(24, 6, NEO_GRB + NEO_KHZ800);


float pos = 0;
int increment = 1;

String data = "";

unsigned long prevTimeTones = millis();
unsigned long prevTimeServos;
unsigned long prevTimeLight;

float currentTime;


long intervalTones = 200;
long intervalServos = 15;
long intervalLight = 1000;

int currentTonePin = 4;
double currentToneFrequency = 523;
double currentToneDuration = 300;
int triggerTime = 10000;

unsigned long start = 0;
unsigned long current = 0;
boolean isTriggering = false;



void setup() {
  servo1.attach(9);
  servo2.attach(8);
  pinMode(5, OUTPUT);
  pinMode(6, OUTPUT);
  pinMode(7, OUTPUT);
  pixels.begin();
  Serial.begin(9600);
}

void loop() {
  current = millis();

  if (Serial.available()) {
    data = Serial.readStringUntil('\r');
    if (data.startsWith("T")) {
      isTriggering = true;
      alertOn();
      start = current;
    }
  }

  if (isTriggering && ((current - start) >= triggerTime)) {
    isTriggering = false;
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
      static bool isWhite = false;
      for (int i = 0; i < pixels.numPixels(); i++) {
        if (isWhite) {
          pixels.setPixelColor(i, pixels.Color(255, 0, 0));
        } else {
          pixels.setPixelColor(i, pixels.Color(255, 255, 255));
        }
      }
      pixels.show();
      isWhite = !isWhite;
    }
  } while (isTriggering == true);
}
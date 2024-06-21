/*
 * Author: Maxwell Robati
 * Version: 10/06/2024
 * Purpose: Physical alert system.
 */

#pragma once

#include <Adafruit_NeoPixel.h>
#include <Adafruit_TiCoServo.h>

const int SERVO_PIN = 9;
#define LED_PIN 6
Adafruit_TiCoServo servo1;
Adafruit_NeoPixel pixels(24, LED_PIN, NEO_GRB + NEO_KHZ800);

const int SPEAKER_DURATION_1 = 300;
const int SPEAKER_DURATION_2 = 200;
const int SPEAKER_FREQUENCY_1 = 523;
const int SPEAKER_FREQUENCY_2 = 440;
const int SPEAKER_PIN_1 = 3;
const int SPEAKER_PIN_2 = 4;

int currentTonePin = SPEAKER_PIN_1;
unsigned long previousSpeakerTime = millis();
long speakerInterval = 200;

unsigned long previousServoTime;
int servoDirection = 1;
long servoInterval = 15;
float servoPosition = 0;

unsigned long previousLightTime;
long lightInterval = 50;
int LEDPosition = 0;

int alertDuration = 10000;
String data = "";
unsigned long alertStartTime = 0;
float currentTime;

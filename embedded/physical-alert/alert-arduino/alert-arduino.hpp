/*
 * Author: Maxwell Robati
 * Version: 9/06/2024
 * Purpose: Physical alert system.
 */

#pragma once

#include <Adafruit_NeoPixel.h>
#include <Adafruit_TiCoServo.h>

Adafruit_TiCoServo servo1;
Adafruit_TiCoServo servo2;
Adafruit_NeoPixel pixels(24, 6, NEO_GRB + NEO_KHZ800);

float pos = 0;
float currentTime;

double currentToneFrequency = 523;
double currentToneDuration = 300;

int increment = 1;
int currentTonePin = 4;
int triggerTime = 10000;

String data = "";

unsigned long prevTimeTones = millis();
unsigned long prevTimeServos;
unsigned long prevTimeLight;
unsigned long start = 0;
unsigned long current = 0;

long intervalTones = 200;
long intervalServos = 15;
long intervalLight = 1000;

boolean isTriggering = false;

/*
 * Author: Toby Connor-Kebbell & Maxwell Robati
 * Version: 9/06/2024
 * Purpose: Gather data from pressure sensor, transmit it to the WiFi card.
 */

#pragma once

#include <SparkFun_LPS28DFW_Arduino_Library.h>
#include <Wire.h>
#include <SoftwareSerial.h>

// Software Serial pins.
#define RXPin 2
#define TXPin 3

// Calibration pins.
#define CAPin 7
#define CWPin 8

void init_sensor(LPS28DFW sensor);
float get_pressure(LPS28DFW sensor);

boolean calibratingAir;
boolean calibratingWater;

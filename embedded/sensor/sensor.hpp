/*
 * Author: Toby Connor-Kebbell & Maxwell Robati
 * Version: 9/06/2024
 * Purpose: Gather data from pressure sensor, transmit it to the WiFi card.
 */

#pragma once

#include <SparkFun_LPS28DFW_Arduino_Library.h>
#include <Wire.h>
#include <SoftwareSerial.h>

#define WifiReadPin 2
#define WifiWritePin 3

#define AirCalibrationPin 7
#define WaterCalibrationPin 8

void init_sensor(LPS28DFW sensor);
float get_pressure(LPS28DFW sensor);

boolean airCalibrationPressed;
boolean waterCalibrationPressed;

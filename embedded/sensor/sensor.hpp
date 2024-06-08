/*
 * Author: Toby Connor-Kebbell & Maxwell Robati
 * Version: 12/05/2024
 * Purpose: Gather data from pressure sensor
 */
#pragma once

#include <SparkFun_LPS28DFW_Arduino_Library.h>
#include <Wire.h>
#include <SoftwareSerial.h>

#define RXPin 2
#define TXPin 3
#define CAPin 7
#define CWPin 8

void init_sensor(LPS28DFW sensor);
float get_pressure(LPS28DFW sensor);

boolean calibratingAir;
boolean calibratingWater;
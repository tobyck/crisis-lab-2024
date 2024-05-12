#pragma once

#include <SparkFun_LPS28DFW_Arduino_Library.h>
#include <Wire.h>
#include <SoftwareSerial.h>

#define RXPin 2
#define TXPin 3

void init_sensor(LPS28DFW sensor);
float get_pressure(LPS28DFW sensor);

/*
 * Author: Maxwell Robati
 * Version: 9/06/24
 * Purpose: Sends sensor data via WiFi card to MQTT broker.
 */

#pragma once

#include <ArduinoMqttClient.h>
#include <ESP8266WiFi.h>
#include <SoftwareSerial.h>

void handleMessage(int messageSize);
void sendData(char msg[20]);

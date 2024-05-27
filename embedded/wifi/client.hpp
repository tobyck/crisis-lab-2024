/*
 * Author: Maxwell Robati
 * Version: 23/05/24
 * Purpose: Sends sensor data via WiFi card to relay server.
 */
#pragma once

#include <ArduinoMqttClient.h>
#include <ESP8266WiFi.h>
#include <SoftwareSerial.h>

void handleMessage(int messageSize);
void sendData(char msg[20]);

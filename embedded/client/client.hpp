/*
 * Author: Maxwell Robati
 * Version: 12/05/24
 * Purpose: Sends sensor data via WiFi card to relay server.
 */
 #pragma once

#include <ArduinoWebsockets.h>
#include <ESP8266WiFi.h>
#include <SoftwareSerial.h>

using namespace websockets;

void handleMessage(WebsocketsMessage message);
void handleEvent(WebsocketsClient &client, WebsocketsEvent event, String data);
void sendData(char msg[20]);
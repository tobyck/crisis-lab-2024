/*
 * Author: Maxwell Robati
 * Version: 9/06/24
 * Purpose: Sends sensor data via WiFi card to the MQTT broker.
 */

#include "wifi.hpp"

// Change these to the appropriate values, _don't_ commit them
const char* wifiName = "";
const char* wifiPassword "";

const char* mqttUsername = "";
const char* mqttPassword = "";

WiFiClient wifiClient;
MqttClient mqttClient(wifiClient);

const char* brokerIPAddress = "170.64.254.27";
const char* mqttTopic = "data";
int mqttPort = 1883;

// All prints to Serial are for debugging purposes

void setup() {
	Serial.setDebugOutput(true);
	Serial.begin(9600);

	while (!Serial); // Wait for serial to begin

	Serial.print("Attempting to connect to WPA SSID: ");
	Serial.println(wifiName);
	WiFi.begin(wifiName, wifiPassword);

	// Wait to connect, abort after 15 seconds.
	for (int i = 0; i < 15 && WiFi.status() != WL_CONNECTED; i++) {
		Serial.print(".");
		delay(1000);
	}
  
	if (WiFi.status() != WL_CONNECTED) {
		Serial.println("No WiFi!");
		return;
	}

	Serial.print("\nWiFi connected, Connecting to MQTT broker: ");
	Serial.println(brokerIPAddress);
  
	mqttClient.setUsernamePassword(mqttUsername, mqttPassword);

	if (!mqttClient.connect(brokerIPAddress, mqttPort)) {
		Serial.print("MQTT connnection failed! Error code: ");
		Serial.println(mqttClient.connectError());
		return;
	}
  
	Serial.println("Connected!");
}

void sendDataToMQTT(char msg[20]) {
	mqttClient.beginMessage(mqttTopic);
	mqttClient.print(msg);
	mqttClient.endMessage();

	Serial.print("Sent data: ");
	Serial.println(msg);
}

char serialBuffer[20] = "";

void loop() {
	// Call poll to keep the server alive, avoids being disconnected by the broker.
	mqttClient.poll();

	while (Serial.available()) {
		for (int i = 0; i < 20; i++) serialBuffer[i] = 0;
		// Store up to 20 bytes from Arduino into buffer, terminating at a newline.
		Serial.readBytesUntil('\n', serialBuffer, 20);

		serialBuffer[strlen(serialBuffer) - 1] = '\0'; // Remove trailing newline

		if (strlen(serialBuffer) != 0) sendDataToMQTT(serialBuffer);
	}

	delay(40);
}

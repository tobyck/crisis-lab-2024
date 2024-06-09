/*
 * Author: Maxwell Robati
 * Version: 9/06/24
 * Purpose: Sends sensor data via WiFi card to the MQTT broker.
 */

#include "wifi.hpp"

const char* wifissid = "";  // Change what's inside the "" to your WiFi Name
const char* wifipass = "";  // "                                 " WiFi Password 

// Broker username and password for the sensor.
const char* mqttuser = "";
const char* mqttpass = "";

WiFiClient wifiClient;
MqttClient mqttClient(wifiClient);

const char* broker = "170.64.254.27";
int         port   = 1883;
const char* topic = "data";

/*
 *	Init.
 */

void setup() {
	// Begin the serial
	Serial.setDebugOutput(true);
	Serial.begin(9600);
	while(!Serial) {
		; // Wait for serial to begin, only needed for using USB Port.
	}

	// Attempt to connect to WiFi
	Serial.print("Attempting to connect to WPA SSID: ");
	Serial.println(wifissid);
	WiFi.begin(wifissid, wifipass);

	// Wait to connect, abort after 15 seconds.
	for(int i = 0; i < 15 && WiFi.status() != WL_CONNECTED; i++) {
		// Retry
		Serial.print(".");
		delay(1000);
	}
  
	// Couldn't connect to WiFi, abort.
	if(WiFi.status() != WL_CONNECTED) {
		Serial.println("No WiFi!");
		return;
	}
  
	// Connected to WiFi
	Serial.print("\nWiFi connected, Connecting to MQTT broker: ");
	Serial.println(broker);
  
	// Set MQTT username and password for the sensor.
	mqttClient.setUsernamePassword(mqttuser, mqttpass);

	// Try to connect to the broker
	if(!mqttClient.connect(broker, port)) {
		Serial.print("MQTT connnection failed! Error code: ");
		Serial.println(mqttClient.connectError());
		return;
	}
  
	Serial.println("Connected!");
}

/*
 *	Send data to MQTT broker over WiFi.
 */

void sendData(char msg[20]) {
	mqttClient.beginMessage(topic);
	mqttClient.print(msg);
	mqttClient.endMessage();
	// Debug logging
	Serial.print("Sent data: ");
	Serial.println(msg);
}

/*
 *	Main loop.
 */

void loop() {
	// Call poll to keep the server alive, avoids being disconnected by the broker.
	mqttClient.poll();

	// Get Sensor data from Arduino.
	char buffer[20] = "";
	if(Serial.available()) {
		// Store up to 20 bytes from Arduino into buffer, terminating at a newline.
		Serial.readBytesUntil('\n', buffer, 20);

		// Remove \n
		buffer[strlen(buffer)-1] = '\0';
	}

	// If the buffer has data
	if(strlen(buffer) != 0) {
		sendData(buffer); // Send pressure sensor data to MQTT broker.
	}

	delay(40);	// Run at 25Hz.
}

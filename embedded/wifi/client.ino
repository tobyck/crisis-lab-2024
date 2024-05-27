/*
 * Author: Maxwell Robati
 * Version: 23/05/24
 * Purpose: Sends sensor data via WiFi card to relay server.
 */
#include "client.hpp"

const char* ssid = "";  // Change what's inside the "" to your Wifi Name
const char* pass = "";  // "                                 " Wifi Password 

WiFiClient wifiClient;
MqttClient mqttClient(wifiClient);

// inTopic is just for testing
const char* broker = "test.mosquitto.org";
int         port   = 1883;
const char* inTopic = "sensor/in";
const char* outTopic = "sensor/out";

void setup() {
  // Begin the serial
  Serial.setDebugOutput(true);
  Serial.begin(9600);
  while(!Serial) {
    ; // Wait for serial to begin, only needed for using USB Port.
  }

  // Attempt to connect to WiFi
  Serial.print("Attempting to connect to WPA SSID: ");
  Serial.println(ssid);

  // Wait to connect, abort after 15 seconds.
  for(int i = 0; i < 15 && WiFi.begin(ssid, pass) != WL_CONNECTED; i++) {
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
  
  // Try to connect to the broker
  if(!mqttClient.connect(broker, port)) {
    Serial.print("MQTT connnection failed! Error code: ");
    Serial.println(mqttClient.connectError());
    return;
  }
  
  Serial.println("Connected!");
  mqttClient.onMessage(handleMessage);
  mqttClient.subscribe(inTopic);
}

// Handle messages from server
void handleMessage(int messageSize) {
  // Log message topic
  Serial.print("Got message with topic: '");
  Serial.print(mqttClient.messageTopic());
  Serial.print(", retained? ");
  Serial.print(mqttClient.messageRetain() ? "true" : "false");
  Serial.print("', length: ");
  Serial.print(messageSize);
  
  Serial.println(", bytes:");
  // Log message contents
  while(mqttClient.available()) {
    Serial.print((char)mqttClient.read());
  }
  Serial.println();
}

// Send data to relay server over WiFi
void sendData(char msg[20]) {
  mqttClient.beginMessage(outTopic);
  mqttClient.print(msg);
  mqttClient.endMessage();
}

void loop() {
  // Call poll to keep the server alive, avoids being disconnected by the broker.
  mqttClient.poll();
  // Get Sensor data from Arduino
  char buffer[20] = "";
  if(Serial.available()) {
    // Store up to 20 bytes from Arduino into buffer, terminating at a newline
    Serial.readBytesUntil('\n', buffer, 20);
    // Remove \n
    buffer[strlen(buffer)-1] = '\0';
  }

  // Keep connection open
  if(mqttClient.available()) {
    // If the buffer has data, send it.
    if(strlen(buffer) != 0) {
      sendData(buffer); // Send Pressure sensor data to Relay server
    }
  }
  delay(500);
}

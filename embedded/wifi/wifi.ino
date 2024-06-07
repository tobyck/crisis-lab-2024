/*
 * Author: Maxwell Robati
 * Version: 6/06/24
 * Purpose: Sends sensor data via WiFi card to relay server.
 */
#include "wifi.hpp"

const char* wifissid = "OPPO A17";  // Change what's inside the "" to your Wifi Name
const char* wifipass = "password";  // "                                 " Wifi Password 

const char* mqttuser = "sensor";
const char* mqttpass = "rVcL1OjYHeJApPsA4fT9";		// remove this shit before committing

WiFiClient wifiClient;
MqttClient mqttClient(wifiClient);

// inTopic is just for testing
const char* broker = "170.64.254.27";
int         port   = 1883;
const char* topic = "data";

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
  
  // Set mqtt user and pass for sensor
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
*/
// Send data to relay server over WiFi
void sendData(char msg[20]) {
  mqttClient.beginMessage(topic);
  mqttClient.print(msg);
  mqttClient.endMessage();
  Serial.print("Sent data: ");
  Serial.println(msg);
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

  // If the buffer has data, send it.
  if(strlen(buffer) != 0) {
    sendData(buffer); // Send Pressure sensor data to Relay server
  }
  delay(40);
}

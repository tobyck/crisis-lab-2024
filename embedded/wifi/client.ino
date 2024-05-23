/*
 * Author: Maxwell Robati
 * Version: 12/05/24
 * Purpose: Sends sensor data via WiFi card to relay server.
 */
#include "client.hpp"

const char* ssid = "";      // Change what's inside the "" to your Wifi Name
const char* password = "";  // "                                 " Wifi Password 
const char* websockets_server_host = "192.168.1.39";  // Server hostname
const uint16_t websockets_server_port = 80;           // Server port

WebsocketsClient client;

void setup() {
  // Begin the serial
  Serial.setDebugOutput(true);
  Serial.begin(9600);

  // Connect to wifi
  WiFi.begin(ssid, password);

  // Wait to connect, abort after 15 seconds.
  for(int i = 0; i < 15 && WiFi.status() != WL_CONNECTED; i++) {
    Serial.print(".");
    delay(1000);
  }

  if(WiFi.status() != WL_CONNECTED) {
    Serial.println("No WiFi!");
    return;
  }

  Serial.println("\nWiFi connected, Connecting to server");
  //Serial.println(WiFi.localIP());   // WiFi IP address
  // Try to connect to WebSocket Server
  bool connected = client.connect(websockets_server_host, websockets_server_port, "/");
  if(connected) {
    Serial.println("Connected!");
    client.send("Hello server!");
  } else {
    Serial.println("Not Connected.");
  }
  client.onMessage(handleMessage);
  client.onEvent(handleEvent);
}

// Handle messages from server
void handleMessage(WebsocketsMessage message) {
  auto data = message.data();
  // Log message
  Serial.print("Got message: ");
  Serial.println(data);
}

// Handle events
void handleEvent(WebsocketsClient &client, WebsocketsEvent event, String data) {
  if(event == WebsocketsEvent::ConnectionClosed) {
    Serial.println("Connection closed.");
  }
}

// Send data to relay server over WiFi
void sendData(char msg[20]) {
  client.send(msg);
}

void loop() {
  // Get Sensor data from Arduino
  char buffer[20] = "";
  if(Serial.available()) {
    // Store up to 20 bytes from Arduino into buffer, terminating at a newline
    Serial.readBytesUntil('\n', buffer, 20);
    // Remove \n
    buffer[strlen(buffer)-1] = '\0';
  }

  // Keep connection open
  if(client.available()) {
    client.poll();    // Poll the server
    // If the buffer has data, send it.
    if(strlen(buffer) != 0) {
      sendData(buffer); // Send Pressure sensor data to Relay server
    }
  }
  delay(500);
}
#include <SoftwareSerial.h>

#define RXPin 2
#define TXPin 3

// Virtual Serial to output to WiFi card, Recieve pin 2, Transmit pin 3.
SoftwareSerial outputSerial = SoftwareSerial(RXPin, TXPin);

void setup() {
  Serial.begin(9600);
  outputSerial.begin(9600);
}

void loop() {
  outputSerial.println("Hello from Arduino");
  delay(500);
}
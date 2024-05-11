int br = 0;
void setup() {
  Serial.setDebugOutput(true);
  Serial.begin(9600);
  br = Serial.baudRate();
}

void loop() {
  Serial.printf("Serial is %d bps\n", br);
  delay(100);
}
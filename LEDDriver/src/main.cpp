#include <Arduino.h>

void setup() {
  SerialUSB.begin();
}

void loop() {
  SerialUSB.printf("hello world\n");
  delay(1000);
}

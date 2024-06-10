import serial
import time
import websocket
import json

# Open Serial port
ser = serial.Serial('/dev/ttyACM0')

# Define what should be sent to the Arduino.
hasTriggeredInLast10Seconds = False

def sendTrigger(ser):
  ser.write(b"T" + b'\r')

# Define what should happen on a WebSocket message.
def on_message(ws, message):
    print(message)
    if json.loads(message)["trigger_alert"] == "True": # If the message contains a trigger_alert
        if hasTriggeredInLast10Seconds == False:
          hasTriggeredInLast10Seconds = True
          sendTrigger(ser)
          print(ser.readline()) # Print the response from the Arduino (should be "Triggered").

          time.sleep(10)

          hasTriggeredInLast10Seconds = False
          print(ser.readline()) # Print the response from the Arduino (should be "notTriggered").

# Start the WebSocket.
ws = websocket.WebSocketApp("ws://170.64.254.27:8443", on_message=on_message)
ws.run_forever()

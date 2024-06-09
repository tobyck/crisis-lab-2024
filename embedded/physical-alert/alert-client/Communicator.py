import serial
import time
import websocket
import json

# open serial port
ser = serial.Serial('/dev/ttyACM0')
# Define What should be send to the arduino
hasTriggeredInLast10Seconds = False
def sendTrigger(ser):
  ser.write(b"T" + b'\r')
# Define what should happen on a websocket message
def on_message(ws, message):
    print(message)
    if json.loads(message)["trigger_alert"] == "True": # If the message contains a trigger_alert
        if hasTriggeredInLast10Seconds == False:
          hasTriggeredInLast10Seconds = True
          sendTrigger(ser)
          print(ser.readline()) # Print the response from the arduino (should be "Triggered")
          time.sleep(10)
          hasTriggeredInLast10Seconds = False
          print(ser.readline()) # Print the response from the arduino (should be "notTriggered")

ws = websocket.WebSocketApp("ws://170.64.254.27:8443", on_message=on_message)
ws.run_forever()

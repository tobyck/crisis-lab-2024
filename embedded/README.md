# Embedded

This folder contains all the code which runs embedded on a board:

- `sensor/`: This part runs on the Arduino that's connected to the sensor. All it does is read raw data and send give to the WiFi board (an ESP8266).
- `wifi/`: This is the code that runs on the ESP8266 which publishes data to the MQTT broker.
- `alert/`: Code for the physical alerting device.
 
## Setup

All of these instructions will use the [Arduino CLI](https://arduino.github.io/arduino-cli). If you haven't already, install that first.

After any setup that may need to be done first (this will be explained soon) the main steps are compiling and uploading:

1. Compile: `arduino-cli compile --fqbn <FQBN> <DIR_TO_COMPILE>`
2. Upload: `arduino-cli upload -p <PORT> --fqbn <FQBN> <DIR_TO_UPLOAD>`

FQBN stands for Fully Qualified Board Name, and the instructions below will tell you what to use. The port is where you've plugged in the board. This can be found by running `arduino-cli board list`.

### Sensor + WiFi

1. Install board definitions for the SparkFun RedBoard (an Arduino UNO) and the ESP8266:

    ```
    arduino-cli core install arduino:avr
    arduino-cli config add board_manager.additional_urls https://raw.githubusercontent.com/sparkfun/Arduino_Boards/main/IDE_Board_Manager/package_sparkfun_index.json https://arduino.esp8266.com/stable/package_esp8266com_index.json
    arduino-cli core install SparkFun:avr:RedBoard
    arduino-cli core install esp8266:esp8266
    ```

2. Install dependencies:

    If you don't already have this option set, you'll need to enable installing libraries from git repositories:

    ```
    arduino-cli config set library.enable_unsafe_install true
    ```

    Then you can install them with:  

    ```
    arduino-cli lib install --git-url https://github.com/sparkfun/SparkFun_LPS28DFW_Arduino_Library https://github.com/arduino-libraries/ArduinoMqttClient.git
    ```

3. Change the WiFi SSID and password; the MQTT username and password; and the Server IP address and port variables in `/embedded/wifi.ino`
4. Compile and upload `embedded/sensor` using the steps from earlier, with `SparkFun:avr:RedBoard` as the FQBN, and `embedded/wifi` with `esp8266:esp8266:generic`

### Physical Alert System

1. Install dependencies:

    If you don't already have this option set, you'll need to enable installing libraries from git repositories:  

    ```
    arduino-cli config set library.enable_unsafe_install true
    ```
    
    Then you can install them with:  

    ```
    arduino-cli lib install --git-url https://github.com/adafruit/Adafruit_TiCoServo https://github.com/adafruit/Adafruit_NeoPixel
    ```
   
2. Compile and upload `embedded/physical-alerts/alert-arduino` using the steps from earlier, with `arduino:avr:uno` as the FQBN.

3. Navigate to the `embedded/physical-alerts/alert-client` directory.

4. Install python dependencies.

   ```
   pip install websockets
   ```

5. Change `ser` to whichever port you are using to communicate with the Arduino, and change the WebSocket IP adress and port, in `Communicator.py`

6. Run `./Communicator.py` using python, this will not work without an internet connection.


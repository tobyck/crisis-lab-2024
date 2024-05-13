# [CRISiSLab](https://www.crisislab.org.nz/crisislabchallenge) 2024

_Created by [me](https://github.com/tobyck), [chunkybanana](https://github.com/chunkybanana), [Alex Berry](https://github.com/AlexBerry0), and [aketon08](https://github.com/aketon08)._

This repository contains all the code for our CRISiSLab 2024 project.

The project has four main components:
 - The dashboard with graphs, information, and alerts. Written in Vue with vue-chartjs for the graphs.
 - The alert system; When a tsunami is detected, an Instagram bot automatically posts a warning, and an e-mail bot sends out emails to people on the mailing list.
 - An Arduino attached to a pressure sensor to read data and send it through a WebSocket to the relay server.
 - A server to take the raw data from the Arduino; do calculations to determine wave height; relay the data to all instances of the dashboard and maintain a cache for redundancy; and determine whether or not there should be an alert, and if so, inform the dashboard and other components of the project.


## Setup

First, clone this repo:

```
git clone https://github.com/tobyck/crisis-lab-2024.git whs-crisis-lab
cd whs-crisis-lab
```

### Embedded

This method uses [the Arduino CLI](https://arduino.github.io/arduino-cli). If you haven't already installed it, do that first.

1. Install board definitions for the SparkFun Arduino Uno and the ESP8266 WiFi board
    
    ```
    arduino-cli core install arduino:avr
    arduino-cli config add board_manager.additional_urls https://raw.githubusercontent.com/sparkfun/Arduino_Boards/main/IDE_Board_Manager/package_sparkfun_index.json https://arduino.esp8266.com/stable/package_esp8266com_index.json
    arduino-cli core install SparkFun:avr:RedBoard
    arduino-cli core install esp8266:esp8266
    ```

2. Install dependencies

    If you don't already have this option set, you'll need to enable installing libraries from git repositories:

    ```
    arduino-cli config set library.enable_unsafe_install true
    ```

    Then you can install them with:

    ```
    arduino-cli lib install --git-url https://github.com/sparkfun/SparkFun_LPS28DF_Arduino_Library https://github.com/gilmaimon/ArduinoWebsockets
    ```

3. Compile the code for the Arduino (change `embedded/main` to the path relative to wherever you are)

    ```
    arduino-cli compile --fqbn SparkFun:avr:RedBoard embedded/main
    ```

4. Plug in the board and find out where it's attached (you're looking for the port)

    ```
    arduino-cli board list
    ```

    You should see something like this: (Port is `/dev/ttyUSB0`)

    ```
    Port         Protocol Type              Board Name FQBN Core
    /dev/ttyUSB0 serial   Serial Port (USB) Unknown
    ```

5. Upload onto the board

    ```
    arduino-cli upload -p PUT_YOUR_PORT_HERE --fqbn arduino:avr:uno embedded
    ```

> [!NOTE]
> If that last command errors saying you don't have permission, _don't_ just try as root, as (in my experience) it won't be able to find your board definitions. I assume this is because they're installed on a per-user basis. Instead you probaby need to add yourself to the `dialout` group. More detail [here](https://askubuntu.com/a/133244).

6. Compile and upload the code for the WiFi board following through the same steps from `step 4`, but change the FQBN to `esp8266:esp8266:generic`

    ```
    arduino-cli compile --fqbn esp8266:esp8266:generic embedded/client
    arduino-cli upload -p PUT_YOUR_PORT_HERE --fqbn esp8266:esp8266:generic embedded/client
    ```

7. If you want, you can monitor logs in the serial output.

    ```
    arduino-cli monitor -p PUT_YOUR_PORT_HERE --config baudrate=115200
    ```

To verify that everything is working properly you can use a dummy server:

1. Change the SSID and password in `embedded/client/client.ino` to your WiFi SSID and password.

2. Go to `demos/max-fake-ws`, install dependencies and launch server:

    ```
    cd demos/max-fake-ws
    npm install
    npm run dev
    ```

3. Press reset button on WiFi board, and look at the output from the server.

### Relay Server

TODO

### Frontend

Pretty standard stuff: install deps and launch vite.

```
cd frontend
npm install
npm run dev
```

You'll also need to run either the real websocket (TBC) or the fake websocket:
```
cd demos/fake-ws
bun fake-ws.ts
```

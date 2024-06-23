# [CRISiSLab Challenge](https://www.crisislab.org.nz/crisislabchallenge) 2024

_Created by [me](https://github.com/tobyck), [chunkybanana](https://github.com/chunkybanana), [Alex Berry](https://github.com/AlexBerry0), and [aketon08](https://github.com/aketon08)._

This repository contains all the code for our CRISiSLab 2024 project.

To set it up, clone this repo and refer to the instructions in each section below.

Sometimes some more information is contained in READMEs in each section folder.

## Setup

These instructions assume that you're in a clone of this repo and that [Nix](https://nixos.org/download/) is installed.

### Android app

#### Install

1. Download the APK [here](https://github.com/tobyck/crisis-lab-2024/raw/master/androidapp/Crisislab/app/release/app-release.apk).
2. Open the APK from your perfered file browser on android
3. Tap install app

#### Build/Develop

1. Open `androidapp/Crisislab` in Android Studio
2. Change any code needed
3. Build using the inbuilt "Build Signed APK"

### Backend

1. Make a `.env` file in the root directory of the project which specifies the following:

    ```
    # Social media credentials
    IG_USERNAME=
    IG_PASSWORD=
    EMAIL=
    EMAIL_PASSWORD=

    # Password for the social alerts endpoint
    ALERT_PASSWORD=

    # These are for the MQTT broker. Make sure they match what you set when you set up Mosquitto.
    MQTT_USERNAME=
    MQTT_PASSWORD=

    # Port for the relay server's WebSocket to run on
    WS_PORT=8443

    # Paths to files for TLS
    CERT_PATH=
    KEY_PATH=
    ```

2. Enter a dev shell with packages you'll need for the rest of this process:

    ```
    nix develop
    ```

3. Go to `backend/mqtt-broker` and generate a password file for the `sensor` and `server` users:

    ```bash
    mosquitto_passwd -c passwords.txt sensor
    mosquitto_passwd passwords.txt server
    ```

4. Start the broker:

    ```bash
    mosquitto -c mosquitto.conf
    ```

5. Go to `backend/alerts`, install dependencies and start the social alerts:

    ```
    bun install
    bun src/index.ts
    ```

6. Start the relay server by running a Nix derivation:

    ```
    RUST_LOG=info SOCIAL_ALERTS=1 nix run .#relay
    ```

    Don't set `SOCIAL_ALERTS` if you don't want to trigger social media alerts.

### Frontend

Inside the `frontend` directory run:

```
bun install
bun run build
```

This will output a `dist` folder with the bundled site in it. In our case we're hosting it on the same server as the backend which can be done with:

```
sudo bun server.ts
```

(Hosts on 80/443 by default so sudo is required).

### Embedded

All of these instructions will use the [Arduino CLI](https://arduino.github.io/arduino-cli). If you haven't already, install that first.

After any setup that may need to be done first (this will be explained soon), the main steps for each component are:

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
    arduino-cli lib install --git-url https://github.com/sparkfun/SparkFun_LPS28DF_Arduino_Library https://github.com/arduino-libraries/ArduinoMqttClient.git
    ```

3. Change the WiFi SSID and password; the MQTT username and password; and the Server IP address and port variables in `/embedded/wifi.ino`
4. Compile and upload `embedded/sensor` using the steps from earlier, with `SparkFun:avr:RedBoard` as the FQBN, and `embedded/wifi` with `esp8266:esp8266:generic`

#### Physical Alert

Inside the `embedded/physical-alert` directory.

1. Install dependencies:

    ```
    arduino-cli lib install --git-url https://github.com/adafruit/Adafruit_TiCoServo https://github.com/adafruit/Adafruit_NeoPixel
    ```
   
2. Compile and upload `embedded/physical-alerts/alert-arduino` using the steps from earlier, with `arduino:avr:uno` as the FQBN.

3. Navigate to the `embedded/physical-alerts/alert-client` directory.

4. Install nodejs dependencies.

   ```
   npm install
   ```

5. Change the `serialport` path to whichever port you are using to communicate with the Arduino, and change the WebSocket IP adress and port, in `client.js`

6. Run `node client.js`, this will not work without an internet connection. THIS WILL ALSO NOT RUN WITH BUN.

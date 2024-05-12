# [CrisisLab](https://www.crisislab.org.nz/crisislabchallenge) 2024

_Created by [me](https://github.com/tobyck), [chunkybanana](https://github.com/chunkybanana), [Alex Berry](https://github.com/AlexBerry0), and [aketon08](https://github.com/aketon08)._

This repository contains all the code for our CrisisLab 2024 project.

The project has three main components:
 - An Arduino attached to a pressure sensor to read data and send it (unprocessed) through a websocket to the relay server. This embedded bit is done in C partly for speed, but mostly because we're lazy.
 - A server to take the raw data from the Arduino, relay the data to all the instances of the dashboard and maintain a cache for rendundency, determine whether or not there should be an alert, and if so, inform the dashboard and other components of the project.
 - The dashboard with graphs, information, and alerts. Written in Vue with vue-chartjs for the graphs.

## Setup

First, obviously, clone this repo:

```
git clone https://github.com/tobyck/crisis-lab-2024.git whs-crisis-lab
cd whs-crisis-lab
```

### Embedded

This method uses [the Arduino CLI](https://arduino.github.io/arduino-cli). If you haven't already installed it, do that first.

1. Install board definitions for the SparkFun board
    
    ```
    arduino-cli core install arduino:avr
    arduino-cli config add board_manager.additional_urls https://raw.githubusercontent.com/sparkfun/Arduino_Boards/main/IDE_Board_Manager/package_sparkfun_index.json
    arduino-cli core install SparkFun:avr:RedBoard
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

3. Compile

    ```
    arduino-cli compile --fqbn SparkFun:avr:RedBoard embedded
    ```

4. Plug in the board and find out where it's attached (you're looking for the port)

    ```
    arduino-cli board list
    ```

    You should see something like this:

    ```
    Port         Protocol Type              Board Name FQBN Core
    /dev/ttyUSB0 serial   Serial Port (USB) Unknown
    ```

5. Upload onto the board

    ```
    arduino-cli upload -p PUT_YOUR_PORT_HERE --fqbn arduino:avr:uno embedded
    ```

> [!NOTE]
> If that last command errors saying you don't have permission, _don't_ just try as root as (in my experience) it won't be able to find your board definitions, I assume because they're installed on a per-user basis. Instead you probaby need to add yourself to the `dialout` group. More detail is [here](https://askubuntu.com/a/133244).

6. Optionally monitor logs

    ```
    arduino-cli monitor -p PUT_YOUR_PORT_HERE --config baudrate=115200
    ```

Once you've verified that everything is working with the steps above, you can use `start.sh` instead. Run the script with no arguments for instructions.

> [!IMPORTANT]
> `start.sh` must be run from inside the `embedded/main/` directory.
> ```
> cd embedded/main
> ./start.sh
> ```

You can test that this works and sends data via WebSocket by:

1. Changing the ssid and password in `embedded/client/client.ino` to your WiFi ssid and password.

2. Plug in WiFi board and upload code

// TODO: Will do this tomorrow

3. Moving into the `demos/max-fake-ws` directory and installing dependencies

    ```
    cd demos/max-fake-ws
    npm install
    ```

4. Run `node servertest.js`

5. Press reset button on WiFi card, and look at the command line output from the NodeJS server.

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

# [CrisisLab](https://www.crisislab.org.nz/crisislabchallenge) 2024

_Created by [me](https://github.com/tobyck), [chunkybanana](https://github.com/chunkybanana), [Alex Berry](https://github.com/AlexBerry0), and [aketon08](https://github.com/aketon08)._

This repository contains all the code for our CrisisLab 2024 project.

The project has three main components:
 - An Arduino attached to a pressure sensor to read data and send it (unprocessed) through a websocket to the relay server. This embedded bit is done in C partly for speed, but mostly because we're lazy.
 - A server to take the raw data from the Arduino, relay the data to all the instances of the dashboard and maintain a cache for rendundency, determine whether or not there should be an alert, and if so, inform the dashboard.
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
    arduino-cli lib install --git-url https://github.com/sparkfun/SparkFun_LPS28DF_Arduino_Library https://github.com/Links2004/arduinoWebSockets
    ```

3. Compile

    ```
    arduino-cli compile --fqbn SparkFun:avr:RedBoard embedded
    ```

4. Plug in the board and find out where it's attached

    ```
    arduino-cli board list
    ```

5. Upload onto the board

    ```
    arduino-cli upload -p PUT_YOUR_PORT_HERE --fqbn arduino:avr:uno embedded
    ```
    
> [!NOTE]
> If that last command errors saying you don't have permission, _don't_ just try as root as (in my experience) it won't be able to find your board definitions, I assume because they're installed on a per-user basis. Instead you probaby need to add youseft to the `dialup` group. More detail is [here](https://askubuntu.com/a/133244).

### Relay Server

TODO

### Frontend

Pretty standard stuff: install deps and launch vite.

```
cd frontend
npm install
npm run dev
```

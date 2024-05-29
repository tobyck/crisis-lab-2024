# MQTT Broker

## Setup

1. Install Mosquitto. If you can't find the right instructions for your system on [the official site](https://mosquitto.org/download/), there are plenty of guides online.

2. Go to `backend/mqtt-broker` and generate a password file for the `sensor` and `server` users:

    ```
    mosquitto_passwd -c passwords.txt sensor
    mosquitto_passwd passwords.txt server
    ```

3. Start the broker:

    ```
    mosquitto -c mosquitto.conf
    ```

4. Test that it works by using `mosquitto_sub` and `mosquitto_pub` (run each in a seperate terminal window):

    ```
    mosquitto_sub -h localhost -t data -u server -P <PASSWORD>
    ```

    ```
    mosquitto_pub -h localhost -t data -u sensor -P <PASSWORD> -m "Hello"
    ```

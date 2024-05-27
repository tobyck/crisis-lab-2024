# MQTT Broker

## Setup

1. Install Mosquitto. If you can't find the right instructions for your system on [the official site](https://mosquitto.org/download/), there are plenty of guides online.

2. Go to this directory (`backend/mqtt-broker`)

3. Generate password file for the `sensor` and `server` users:

    ```
    mosquitto_passwd -c passwords.txt sensor
    mosquitto_passwd passwords.txt server
    ```

4. Generate TLS files for the broker. The way you do this will vary depending on who you want the certificates to be signed by, but there are some scripts to help with this in `backend/mqtt-broker/scripts`. To generate all the files you'll need to test it from nothing, you can use `quickstart.sh` from inside the `mqtt-broker` directory to create a CA and a certificate/key for the broker and two clients.

	The script will ask you several times for "information that will be incorporated into your certificate request." It will ask for this for the CA, the broker and the clients, in that order. Make sure the CA has some different parameters to the broker and clients, and that the common name (CN) of the broker and the clients is either the hostname or domain name of the server that the broker is running on. For testing you can just put `localhost`.

5. Now, you can verify that everything works. (Run each of the following commands in a seperate terminal window).

    1. Start the broker:

        ```
        mosquitto -c mosquitto.conf
        ```

    2. Subscribe to the `test` topic:

        ```
        mosquitto_sub -p 8883 -t test -u server -P <PASSWORD> --cafile tls/ca/ca.crt --cert tls/client1/client.crt --key tls/client1/client.key
        ```

    3. Publish "Hello" to that topic:

        ```
        mosquitto_pub -p 8883 -t test -u sensor -P <PASSWORD> --cafile tls/ca/ca.crt --cert tls/client2/client.crt --key tls/client2/client.key -m "Hello" 
        ```

    The last two commands will ask you for the PEM passphrases you used to encrypt the client's key.

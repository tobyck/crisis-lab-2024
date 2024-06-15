# Setup Instructions

This document explains how to setup all of the backend components on a single server.

## Using [Nix](https://nixos.org/download/)

### Setting the necessary environment variables

Make a `.env` file in the root directory of the project which specifies the following:

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
CERT_PATH=tls/cert.crt
KEY_PATH=tls/cert.key
```

### Mosquitto

1. Go to `backend/mqtt-broker` and generate a password file for the `sensor` and `server` users:

    ```bash
    mosquitto_passwd -c passwords.txt sensor
    mosquitto_passwd passwords.txt server
    ```

2. Start the broker via a dev environment in the Nix flake which has Mosquitto:

    ```bash
    nix develop -c mosquitto -c mosquitto.conf
    ```

### Social Alerts

TODO

### Relay Server

```
RUST_LOG=info SOCIAL_ALERTS=1 nix run .#relay
```

Don't set `SOCIAL_ALERTS` if you don't want to trigger social media alerts.

## Without Nix

Install Nix then refer to instructions above.

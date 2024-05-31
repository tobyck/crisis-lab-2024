# Relay Server

This is responsible for receiving data from the sensor via an MQTT broker, processing the data, determining whether or not there's an alert, caching recent data, and forwarding it to instances of the dashboard.

# Setup

1. Make a `.env` file and specify values for `MQTT_PORT` (1883), `WS_PORT` (whatever you like, but probably 8443), `USERNAME` (this needs to be set to `server`), and `PASSWORD` which should be whatever you set when setting up the MQTT broker.

2. Run the server:

    ```
    cd backend/relay
    cargo run
    ```

    - To run with logs set the `RUST_LOG` environment variable to `debug` before running.
    - To generate an executable use `cargo build`, and use the `--release` flag to optimise it. The executable will be in `target/debug` or `target/release`.
    - If you're done you can use `cargo clean` to remove the target directory.

use std::time::Duration;

// port that the mqtt broker is running on
pub const MQTT_PORT: u16 = 1883;

// port that the websocket will be open on
pub const WS_PORT: u16 = 8443;

// how many times per second that data is being sent from the server
pub const FREQUENCY: u64 = 25;

// capacity of the channel which holds incoming messages for the mqtt client, as
// well as the broadcast channel for communicating the the tasks handling
// websocket connections.
pub const CHANNEL_CAPACITY: usize = FREQUENCY as usize * 10;

// capacity of the cache of recent data that's sent to client when they first connect
pub const CACHE_CAPACITY: usize = FREQUENCY as usize * 20;

// how many cm tall a wave needs to be to trigger an alert
pub const ALERT_THRESHOLD: f32 = 8.0;

// seconds before another alert can be triggered
pub const ALERT_COOLDOWN: Duration = Duration::from_secs(10);

// endpoint to post alerts to do for social alerts
pub const ALERT_ENDPOINT: &str = "http://localhost:8783/alert";

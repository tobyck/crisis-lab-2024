use std::time::Duration;

// port that the mqtt broker is running on
pub const MQTT_PORT: u16 = 1883;

// topic that the sensor will send data on
pub const MQTT_TOPIC: &str = "data";

// how many times per second that data is being sent from the server
pub const FREQUENCY: usize = 25;

// capacity of the channel which holds incoming messages for the mqtt client, as
// well as the broadcast channel for communicating the tasks handling
// websocket connections.
pub const CHANNEL_CAPACITY: usize = FREQUENCY * 10;

// capacity of the cache of recent data that's sent to client when they first connect
pub const CACHE_CAPACITY: usize = FREQUENCY * 20;

// seconds before another alert can be triggered
pub const ALERT_COOLDOWN: Duration = Duration::from_secs(10);

// the sensor has a metal weight attached to the bottom, so this value will be subtracted from
// computed heights to account for that (in cm)
pub const SENSOR_HEIGHT_FROM_FLOOR: f32 = 1.0; // TODO: Get an accurate measurement for this

// maximum time of no messages from sensor before the sensor's status becomes "offline"
pub const MAX_SENSOR_DOWNTIME: Duration = Duration::from_secs(1);

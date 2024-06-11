use std::env;

use serde_json::json;
use log::{error, info};

use crate::{config::{ALERT_COOLDOWN, ALERT_THRESHOLD}, data::{Alert, SharedAlertsVec, SharedCache}};

async fn social_alert(height: f32) {
    let password = env::var("ALERT_PASSWORD")
        .expect("Error reading ALERT_PASSWORD environment variable");

    let body = json!({
        "height": height,
        "password": password
    });

    let alert_endpoint = env::var("ALERT_ENDPOINT")
        .expect("Error reading ALERT_ENDPOINT environment variable");

    info!("Posting alert to social alerts system");

    // send the above json the social alerts system
    let client = reqwest::Client::new();
    let response = client.post(&alert_endpoint)
        .json(&body)
        .send()
        .await;

    if let Err(error) = response {
        error!("Error trying to post alert to {}: {}", &alert_endpoint, error);
    }
}

pub async fn check_for_alert(
    current_wave_height: f32,
    cache: &SharedCache,
    alerts: &SharedAlertsVec
) -> Option<Alert> {
    let previous_data_packet = cache.read().await.last();

    let previous_wave_height: Option<f32> = match previous_data_packet {
        Some(data_packet) => data_packet.get_height(),
        None => None
    };

    if let Some(previous_wave_height) = previous_wave_height {
        let mut alerts_lock = alerts.write().await;

        let cooldown_complete = match alerts_lock.last() {
            Some(alert) => alert.timestamp.elapsed() > ALERT_COOLDOWN,
            None => true
        };

        let should_trigger_alert = 
            previous_wave_height >= ALERT_THRESHOLD &&
            current_wave_height < previous_wave_height &&
            cooldown_complete;

        if !should_trigger_alert {
            return None;
        }

        let alert = Alert {
            height: previous_wave_height,
            // this .unwrap() is fine because of the checks above
            timestamp: previous_data_packet.unwrap().get_timestamp()
        };

        // save the alert so clients can see when they reconnect
        alerts_lock.push(alert.clone());

        // set ALERTS to enable social media alerts
        if env::var("ALERTS").is_ok() {
            social_alert(previous_wave_height).await;
        }

        return Some(alert);
    }

    None
}

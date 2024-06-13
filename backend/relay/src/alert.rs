use std::env;

use serde_json::json;
use log::{debug, error, info};

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

// this function assumes that the data packet containing the current wave height hasn't been cached
pub async fn check_for_alert(
    cache: &SharedCache,
    alerts: &SharedAlertsVec
) -> Option<Alert> {
    let cache_lock = cache.read().await;
    
    if cache_lock.len() < 2 {
        return None;
    }

    // we use -2 here because the very last data packet is the current one
    if let Some(previous_data_packet) = cache_lock.at(cache_lock.len() - 2) {
        if let Some(previous_wave_height) = previous_data_packet.get_height() {
            let current_wave_height = cache_lock.at(cache_lock.len() - 1)
                .unwrap() // we know that there was an item before so there must be one here
                .get_height()
                .expect("The most recent data packet had no previous data, yet the one before it did. This should not be possible");

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

            info!("Tsunami detected. Triggering alert");

            let alert = Alert {
                height: previous_wave_height,
                timestamp: previous_data_packet.get_timestamp()
            };

            // store the alert so it can be sent when a client first connects
            alerts_lock.push(alert.clone());

            // set the SOCIAL_ALERTS environment variable to enable social media alerts
            if env::var("SOCIAL_ALERTS").is_ok() {
                social_alert(previous_wave_height).await;
            }

            return Some(alert);
        }
    }

    debug!("No previous data, not alerting");

    None
}

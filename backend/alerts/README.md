# Alerts

This has a HTTP server running on port `8783`:

- `GET /subscribe?email=[email]` - Subscribes an email to the alerts
- `GET /unsubscribe?uuid=[email uuid]` - Unsubscribes an email from the alerts by the associated uuid, sent in the email
- `POST /alert` - Sends an alert to all subscribed emails and to the instagram account. Body should be a JSON object containing a `password` and a `height`.

To configure this, you need to set the following environment variables:
- `EMAIL` - The email address to send alerts from
- `EMAIL_PASSWORD` - The password for the email address
- `IG_USERNAME` - The username for the Instagram account
- `IG_PASSWORD` - The password for the Instagram account
- `ALERT_PASSWORD` - The password required to send alerts

To build, run `bun install`. To run, run `bun src/index.ts`.
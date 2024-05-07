# Plan

## Todo

- [ ] Get arduino dev environment setup
- [ ] Handle data from sensor and sending it on a WS
- [ ] Design frontend on paper

## Outline

### Frontend

Tools:
    - Vue
    - vue-chartjs

Content:
    - Graphs of: pressure and wave height over time
    - Status info for relay, sensor, uptime, etc.
    - Recent events
    - Alert

### Embedded

Tools:
    - Rip-off arduino uno
    - C with WS api

- Handles the sensor data and sending it to the relay server
- Controls motors on the physical alerting device
- Need to keep as stripped down as possible; only 32kb of flash memory

### Relay Server

- Can prob just quickly hack something together with TS
- Cache last 5min or so in case something breaks (reliability as relevant impl.)
- Also decides when to activate alerts

# Plan

## Todo

- Embedded
    - [X] Get arduino dev environment setup
    - [ ] Figure out how to install library for the wifi board
    - [ ] Make sure the WS lib can fit on the board
    - [ ] Handle data from sensor and sending it on a WS
- Frontend
    - [ ] Design on paper

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

### Alert

- Instagram bot
- Email bot (Maybe)
- Siren
- Wave display with servers 

## Dates things will be done by

### 14/5/24

- dashboard recieves (fake) data, displays 2 graphs, pressure over time and wave height over time
- Forwarding bit of websocket is done
- Ardunio can send data in a decent format

### 21/5/24

- Dashboard works
- Past incidents tab works
- Relay sever done
- Detecting alerts on server, and seeing them on dashboard

### 28/5/24

- Instagram bot working
- RSS feed is working

### 4/6/24

- Physical alert system done

import { reactive, ref, computed } from 'vue';
import { THEME } from './theme.js';

export let packetData = reactive([]);

export let logs = reactive([]);

export const loaded = ref(false);

export let calibrations = {};

let LOCAL = true;

export async function initWebsocket() {
    let ws = new WebSocket(LOCAL ? 'ws://localhost:8443' : 'wss://dashboard.alex-berry.net:8443');
    ws.addEventListener('message', message => {
        const data = JSON.parse(message.data);

        lastRelayMessage.value = Date.now();

        if (loaded.value == false) { // handle initial data packet
            loaded.value = true;
            calibrations = {
                ...data.calibrations,
                threshold: data.alert_threshold
            }
            packetData.push(...data.previous_data);
            logs.push(...data.previous_alerts.map(stringifyIncident).reverse());
            if (packetData.length < 500) {
                packetData.unshift(...Array(500 - packetData.length).fill(null));
            }
        } else if (data.pressure) { // regular data packet
            packetData.shift();
            packetData.push(data);

            lastSensorMessage.value = Date.now();
        } else if (data.sensor_offline) {
            // don't need to do anything, just update the last message time
        } else if (data.height) { // alert packets have height but not pressure
            logs.unshift(stringifyIncident(data));

            THEME.alertActive = true;
            setTimeout(() => {
                THEME.alertActive = false;
            }, 10000);
        } else if (data.test_timestamp) { // for testing purposes, probably does nothing
            console.info('Timestamp', Date.now() - data.test_timestamp, 'ms')
        }
    })
}

let stringifyIncident = ({ timestamp, height }) => `${Intl.DateTimeFormat('en-GB', {
    dateStyle: 'short',
    timeStyle: 'long',
    timeZone: 'Pacific/Auckland',
}).format(new Date(timestamp))
    .replace(',', '').replace(/ GMT+.*/, '')
    .replace(/(..\/..\/)..(..) (.*)/, '[$3 $1$2]')
    }
    ${height.toFixed(2)}cm tsunami detected`;

// basically updating the online status every 40ms, because reactivity's weird
let currentTime = ref(Date.now());
setInterval(() => {
    currentTime.value = Date.now();
}, 40);

// this is a custom websocket for checking if alerts is online, pings once a second
let ws = new WebSocket('wss://dashboard.alex-berry.net:8783/ws');

let lastAlertMessage = ref(0), lastSensorMessage = ref(0), lastRelayMessage = ref(0);;

ws.onmessage = () => {
    lastAlertMessage.value = Date.now();
};

export let online = {
    alert: computed(() => currentTime.value - lastAlertMessage.value < 2000),
    server: computed(() => currentTime.value - lastRelayMessage.value < 2000),
    sensor: computed(() => currentTime.value - lastSensorMessage.value < 2000),
}
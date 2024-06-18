import { reactive, ref, computed } from 'vue';
import { THEME } from './theme.js';

export let packetData = reactive([]);

export let logs = reactive([]);

export const loaded = ref(false);

export let calibrations = {};

let LOCAL = false;

let lastSensorMessage = ref(0), lastRelayMessage = ref(0);

export async function initWebsocket() {
    let ws = new WebSocket(LOCAL ? 'ws://localhost:8443' : 'wss://dashboard.alex-berry.net:8443');
    ws.addEventListener('message', message => {
        const data = JSON.parse(message.data);

        lastRelayMessage.value = Date.now();

        if (loaded.value == false) { // init packet
            loaded.value = true;
            calibrations = data.calibrations;
            packetData.push(...data.previous_data);
            logs.push(...data.previous_alerts.map(stringifyIncident).reverse());
            if (packetData.length < 500) {
                packetData.unshift(...Array(500 - packetData.length).fill(null));
            }
        } else if (data.pressure) { // data packet
            packetData.shift();
            packetData.push(data);

            lastSensorMessage.value = Date.now();
        } else if (data.sensor_offline) {

        } else { // alert packet
            logs.unshift(stringifyIncident(data));

            THEME.alertActive = true;
            setTimeout(() => {
                THEME.alertActive = false;
            }, 10000);
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

let currentTime = ref(Date.now());
setInterval(() => {
    currentTime.value = Date.now();
}, 40);

// this is a custom websocket for checking if alerts is online, pings once a second
let ws = new WebSocket('wss://dashboard.alex-berry.net:8783/ws');

let lastAlertMessage = ref(0);

ws.onmessage = () => {
    lastAlertMessage.value = Date.now();
};

export let online = {
    alert: computed(() => currentTime.value - lastAlertMessage.value < 2000),
    server: computed(() => currentTime.value - lastRelayMessage.value < 2000),
    sensor: computed(() => currentTime.value - lastSensorMessage.value < 2000),
}
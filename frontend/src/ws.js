import { reactive, ref } from 'vue';
import { THEME } from './theme.js';

export let packetData = reactive([]);

export let logs = reactive([]);

export const loaded = ref(false);

let LOCAL = true;

export async function initWebsocket() {
    let ws = new WebSocket(LOCAL ? 'ws://localhost:8443' : 'ws://170.64.254.27:8443');
    ws.addEventListener('message', message => {
        const data = JSON.parse(message.data);
        //console.log(message);

        if (loaded.value == false) { // init packet
            loaded.value = true;
            packetData.push(...data.previous_data);
            logs.push(...data.previous_alerts.map(stringifyIncident).reverse());
            if (packetData.length < 500) {
                packetData.unshift(...Array(500 - packetData.length).fill(null));
            }
        } else if (data.pressure) { // data packet
            packetData.shift();
            packetData.push(data);
            /*if (data.triggerAlert) {
                logs.unshift(stringifyIncident(data));
            }*/
            // TODO: handle alerts
        } else { // alert packet
            logs.unshift(stringifyIncident(data));
        }

        /*
            console.log('ALERT!!!!! WEE WOO WEE WOO')
            THEME.alertActive = true;
            incidents.push(data.data);
            setTimeout(() => {
                THEME.alertActive = false;
            }, 20000);
        */
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
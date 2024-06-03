import { reactive, ref } from 'vue';
import { THEME } from './theme.js';

export let packetData = reactive([]);

export let incidents = reactive([]);

export const loaded = ref(false);

export async function initWebsocket() {
    let ws = new WebSocket('ws://0.0.0.0:8443');
    ws.addEventListener('message', message => {
        const data = JSON.parse(message.data);
        console.log(data);

        if (loaded.value == false) { // init packet
            loaded.value = true;
            packetData.push(...data.previous_data);
            incidents.push(...data.previous_alerts);
            console.log(data);
        } else {
            packetData.shift();
            packetData.push(data);
            // TODO: handle alerts
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
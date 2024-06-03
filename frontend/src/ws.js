import { reactive, ref } from 'vue';
import { THEME } from './theme.js';

export let packetData = reactive([]);

export let incidents = reactive([]);

export const loaded = ref(false);

export async function initWebsocket() {
    let ws = new WebSocket('wss://0.0.0.0:8443');
    ws.addEventListener('message', message => {
        const data = JSON.parse(message.data);
        if (data.type == 'data') { // new packet
            packetData.shift();
            packetData.push(data.data);
        } else if (data.type == 'init') { // initial array
            loaded.value = true;
            packetData.push(...data.data);
            incidents.push(...data.incidents);
            console.log(data);
        } else if (data.type == 'alert') {
            console.log('ALERT!!!!! WEE WOO WEE WOO')
            THEME.alertActive = true;
            incidents.push(data.data);
            setTimeout(() => {
                THEME.alertActive = false;
            }, 20000);
        }
    })
}
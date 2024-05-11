import { reactive, ref } from 'vue';

export let packetData = reactive([]);

export const loaded = ref(false);

export async function initWebsocket () {
    let ws = new WebSocket('ws://localhost:8081');
    ws.addEventListener('message', message => {
        const data = JSON.parse(message.data);
        if ("timeStamp" in data) { // new packet
            packetData.shift();
            packetData.push(data);
        } else { // initial array
            loaded.value = true;
            packetData.push(...data);
            console.log(data);
        }
    })
}
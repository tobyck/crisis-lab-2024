<template>
    <div class="box">
        <div class="header">
            <div class="incidents">Logs</div>
            <div class="status">
                <StatusLight :status="serverOnline" name="Relay" />
                <StatusLight :status="false" name="Sensor" />
                <StatusLight :status="alertOnline" name="Alerts" />
            </div>
        </div>
        <div class="rest">
            <span v-for="log in logs">
                <span>{{ log }}</span>
                <br />
            </span>
            <div class="undetected" v-if="logs.length == 0">No tsunami have been detected yet</div>
        </div>
    </div>
</template>

<style scoped>
div.box {
    height: 100%;
    width: 100%;
    color: v-bind('THEME.textColor');
    box-sizing: border-box;
    overflow-y: scroll;
    background-color: v-bind('THEME.backgroundColor');
    display: flex;
    align-items: stretch;
    flex-direction: column;
    border-radius: 1vw;
    row-gap: 0.5vw;
    padding: 0.75vw;
}

.header {
    height: 50px;
    flex: 0 0 50px;
}

.rest {
    flex: 1;
    flex-basis: 0;
    display: v-bind('logs.length == 0 ? "flex" : "block"');
    align-items: v-bind('logs.length == 0 ? "center" : "flex-start"');
    overflow-y: scroll;
    padding-left: 0.8vw;
    padding-top: 0.8vw;
    padding-bottom: 0.8vw;
    border-radius: 0.5vw;
    background-color: v-bind('THEME.backgroundColor3');
}

.status,
.rest {
    font-family: 'Courier New', Courier, monospace;
}

.status {
    text-align: center;
}

.rest>div {
    width: 100%;
}



div.incidents {
    font-size: 20px;
    text-align: center;
    padding-top: 5px;
    position: sticky;
}

.undetected {
    font-size: 14px;
    text-align: center;
    font-style: italic;
    width: 100%;
}



div.box p {
    margin-left: 10px;
}

div.circle {
    width: 14px;
    height: 14px;
    background-color: v-bind('THEME.borderColor');
    border-radius: 50%;
    display: inline-block;
}

span.alert {
    color: v-bind('THEME.borderColor');
}
</style>

<script setup>
import { logs, packetData, loaded } from '../ws.js';
import { THEME } from '@/theme.js';
import StatusLight from './StatusLight.vue';
import { computed, ref } from 'vue';

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

let alertOnline = computed(() => currentTime.value - lastAlertMessage.value < 2000);


const serverOnline = computed(() => loaded.value && currentTime.value - packetData.at(-1)?.timestamp < 2000);
</script>
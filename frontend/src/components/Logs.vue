<template>
    <div class="box">
        <div class="header">
            <div class="incidents">Logs</div>
            <div class="status">
                <!-- don't question it -->
                <StatusLight :status="{ q: online.server }" name="Relay" />
                <StatusLight :status="{ q: online.sensor }" name="Sensor" />
                <StatusLight :status="{ q: online.alert }" name="Alerts" />
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

@media screen and (min-width: 3000px) {
    div.incidents {
        font-size: 40px;
    }

    div.rest {
        font-size: 30px;
    }

    div.undetected {
        font-size: 20px;
    }
}
</style>

<script setup>
import { logs, online } from '../ws.js';
import { THEME } from '@/theme.js';
import StatusLight from './StatusLight.vue';
</script>
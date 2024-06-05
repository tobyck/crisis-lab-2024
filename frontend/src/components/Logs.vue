<template>
    <div class="box">
        <div class="header">
            <div class="incidents">Logs</div>
            <div class="status">
                <StatusLight :status="true" name="Relay" />
                <StatusLight :status="false" name="Sensor" />
                <StatusLight :status="true" name="Alerts" />
            </div>
        </div>
        <div class="rest">
            <span v-for="log in logs">
                <span>{{ log }}</span>

                <!--<span class='alert' v-if="THEME.alertActive && incident == incidents.at(-1)">occuring</span>
                <span v-else>detected</span>

                <span v-if="THEME.alertActive && Date.now() - incident.timeStamp < 20 * 1000" class="circle"></span>-->
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
    /*width: 38.5vw;*/
    background-color: v-bind('THEME.backgroundColor');
    /*margin: 0.5vw 0.5vw 0.5vw 1vw;*/
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
    /*margin-top: 7px;*/
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
import { logs } from '../ws.js';
import { THEME } from '@/theme.js';
import StatusLight from './StatusLight.vue';
</script>
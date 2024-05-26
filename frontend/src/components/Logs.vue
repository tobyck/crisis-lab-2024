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
            <div v-for="incident in [...incidents].reverse()">
                <span>Tsunami of height {{
                    incident.height.toFixed(2)
                    }} cm
                    <span class='alert' v-if="THEME.alertActive && incident == incidents.at(-1)">occuring</span>
                    <span v-else>detected</span>
                    at {{
                        Intl.DateTimeFormat('en-GB', {
                            dateStyle: 'short',
                            timeStyle: 'long',
                            timeZone: 'Pacific/Auckland',
                        }).format(new Date(incident.timeStamp)).replace(',', '').replace(/ GMT+.*/, '')
                    }}
                    <span v-if="THEME.alertActive && Date.now() - incident.timeStamp < 20 * 1000" class="circle"></span>
                </span>
            </div>
            <div class="undetected" v-if="incidents.length == 0">No tsunami have been detected yet</div>
        </div>
    </div>
</template>

<style scoped>
div.box {
    height: calc(100vh - 80px);
    color: v-bind('THEME.textColor');
    box-sizing: border-box;
    overflow-y: scroll;
    width: 40vw;
    background-color: v-bind('THEME.backgroundColor3');
}

@media screen and (max-width: 900px) {
    div.box {
        width: 100vw !important;
        height: calc((100vh - 80px) / 3) !important;
    }
}

.header {
    height: 50px;
}

.rest {
    height: calc(100% - 50px);
    display: v-bind('incidents.length == 0 ? "flex" : "block"');
    align-items: v-bind('incidents.length == 0 ? "center" : "flex-start"');
}

.status,
.rest {
    font-family: 'Courier New', Courier, monospace;
    text-align: center;
}

.rest>div {
    width: 100%;
}



div.incidents {
    font-size: 20px;
    text-align: center;
    /*margin-top: 7px;*/
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
import { incidents } from '../ws.js';
import { THEME } from '@/theme.js';
import StatusLight from './StatusLight.vue';
</script>
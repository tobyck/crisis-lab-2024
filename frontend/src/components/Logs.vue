<template>
    <div class="box">
        <p class="incidents">Logs
            <StatusLight :status="true" name="Relay:" />
        </p>
        <div v-for="incident in [...incidents].reverse()">
            <p>Tsunami of height {{
                incident.height.toFixed(2)
            }} cm 
            <span class='alert' v-if="THEME.alertActive && incident == incidents.at(-1)">occuring</span>
            <span v-else>detected</span>
            at {{ 
                Intl.DateTimeFormat('en-GB', {
                    dateStyle: 'short',
                    timeStyle: 'long',
                    timeZone: 'Pacific/Auckland',
                }).format(new Date(incident.timeStamp)).replace(',','').replace(/ GMT+.*/,'')
            }}
            <div v-if="THEME.alertActive && Date.now() - incident.timeStamp < 20 * 1000" class="circle"></div>
            </p>
        </div>
    </div>
</template>

<style scoped>
div.box {
    border-radius: 25px;
    border: 2px solid v-bind('THEME.borderColor');
    width: 40vw;
    margin-left: 5vw;
    margin-right: 5vw;
    color: v-bind('THEME.textColor');
    min-height: 20vw;
    max-height: 20vw;
    box-sizing: border-box;
    overflow-y: scroll;
}

@media screen and (max-width: 1000px) {
    div.box {
        width: min(90%, 600px);
        margin-left: max(5%, calc((100% - 600px) / 2));
    }
}

@media screen and (max-height: 1000px) {
    div.box {
        height: min(40%, 600px);
    }
}

p.incidents {
    font-size: 20px;
    text-align: center;
    margin-top: 7px;
    position: sticky;
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
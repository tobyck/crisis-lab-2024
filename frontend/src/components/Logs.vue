<template>
    <div class="box">
        <p class="incidents">Past Incidents</p>
        <div v-for="incident in incidents.reverse()">
            <p>Tsunami of height {{
                incident.height.toFixed(2)
            }} cm {{
                THEME.alertActive && Date.now() - incident.timeStamp < 20 * 1000? 'occuring' : 'detected'
            }} at {{ 
                Intl.DateTimeFormat('en-GB', {
                    dateStyle: 'short',
                    timeStyle: 'long',
                    timeZone: 'Pacific/Auckland',
                }).format(new Date(incident.timeStamp)).replace(',','').replace(/ GMT+.*/,'')
            }}
            <span v-if="THEME.alertActive && incident == incidents.at(-1)">ACTIVE</span>
            </p>
        </div>
    </div>
</template>

<style scoped>
div.box {
    border-radius: 25px;
    border: 2px solid v-bind('THEME.borderColor');
    width: 39vw;
    margin-left: 5vw;
    margin-right: 5vw;
    color: v-bind('THEME.textColor');
}

p.incidents {
    font-size: 20px;
    text-align: center;
    margin-top: 7px;
}

div.box p {
    margin-left: 10px;
}

span {
    color: v-bind('THEME.borderColor');
    font-size: 40px;
}
</style>

<script setup>
import { incidents } from '../ws.js';
import { THEME } from '@/theme.js';
</script>
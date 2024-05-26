<template>
    <div class="body">
        <Header />
        <div class="flex">
            <Logs />
            <Chart name="height" :options="{
                y: 'Water level (cm)',
                title: 'Water Level',
                minY: 0,
                maxY: 3,
                color: THEME.graphColor
            }" :data-source="height" />
            <Chart name="pressure" :options="{
                y: 'Pressure (Pa)',
                title: 'Sensor Pressure',
                minY: 1018,
                maxY: 1022,
                color: THEME.graphColor2
            }" :data-source="pressure" />
            <div class="live-view">
                <Chart name="live-view" :options="{
                    y: 'Wave height (cm)',
                    title: 'Live View',
                    minY: 1018,
                    maxY: 1022,
                    color: THEME.graphColor2
                }" :data-source="pressure" />
            </div>
        </div>
        <Footer />
    </div>
</template>

<style scoped>
div.flex {
    justify-content: center;
    align-items: center;
    height: calc(100vh - 80px);
    display: flex;
    flex-flow: column wrap;
    row-gap: 0;
    column-gap: 0;
}

.live-view {
    display: v-bind('THEME.isMobile ? "none" : "block"');
}
</style>


<style>
body {
    margin: 0;
}

div.body {
    font-family: 'DejaVu Sans Mono', 'Courier New', Courier, monospace;
    background-color: v-bind('THEME.backgroundColor');
    min-height: 100vh;
    clear: both;
    user-select: none;
}

@font-face {
    font-family: "DejaVu Sans Mono";
    src: url('DejaVuSansMono.ttf');
}
</style>

<script setup>
import Header from './components/Header.vue';
import Chart from './components/Chart.vue';
import Footer from './components/Footer.vue';
import Logs from './components/Logs.vue';
import { THEME } from './theme';
import { ref, computed } from 'vue';
import { packetData, initWebsocket, loaded } from './ws.js';

initWebsocket();


const pressure = computed(() => ({
    values: packetData.filter(t => t != null)
        .map(({ pressure, timeStamp }) =>
        ({
            x: 20 - (Date.now() - timeStamp) / 1000,
            y: pressure
        })
        ),
    loaded: loaded.value
}))

const height = computed(() => ({
    values: packetData.filter(t => t != null)
        .map(({ waterLevel, timeStamp }) =>
        ({
            x: 20 - (Date.now() - timeStamp) / 1000,
            y: waterLevel
        })
        ),
    loaded: loaded.value
}))
</script>
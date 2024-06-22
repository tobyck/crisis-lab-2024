<!-- This is the main body of the app. It's a bunch of nested flexboxes, yes it's mildly cursed 
    but it's far, far better than the alternative -->
<template>
    <div class="body">
        <div class="header">
            <Header />
        </div>
        <div class="main">
            <div class="alert-container" v-if="THEME.isMobile">
                <AlertDisplay />
            </div>
            <div class="log-box">
                <div class="alert-container" v-if="!THEME.isMobile">
                    <AlertDisplay />
                </div>
                <div class="log-container">
                    <Logs />
                </div>
            </div>
            <div class="chart-container">
                <div class="chart-box">
                    <Chart name="height" :options="{
                        y: 'Water level (cm)',
                        title: 'Water Level',
                        minY: 0,
                        maxY: 3,
                        color: THEME.graphColor
                    }" :data-source="height" />
                </div>
                <div class="chart-box">
                    <Chart name="pressure" :options="{
                        y: 'Pressure (hPa)',
                        title: 'Sensor Pressure',
                        minY: 1018,
                        maxY: 1022,
                        color: THEME.graphColor2
                    }" :data-source="pressure" />
                </div>
            </div>
        </div>
        <div class="footer">
            <Footer />
        </div>
    </div>
    <AlertBackground />
</template>

<style scoped>
div.main {
    justify-content: center;
    align-items: stretch;
    display: flex;
    flex-flow: row;
    row-gap: 0;
    column-gap: 0;
    flex-grow: 1;
    flex-shrink: 1;
    column-gap: 1vw;
    margin-left: 1vw;
    margin-right: 1vw;
}

div.body {
    display: flex;
    flex-flow: column;
    row-gap: 1vw;
}

div.header {
    height: 50px;
}


div.footer {
    height: 20px;
}

div.log-box {
    flex: 2 2 0;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    row-gap: 0.75vw;
}

div.log-container {
    flex: 1 1 0;
    display: flex;
    flex-direction: row;
    align-items: stretch;
}

div.alert-container {
    flex: 0 0 70px;
}

div.chart-container {
    display: flex;
    flex-flow: column;
    flex: 3 3;
    justify-content: center;
    align-items: stretch;
    row-gap: 1vw;
    flex-basis: 0;
}

div.chart-box {
    flex: 1 1 0;
    background-color: v-bind('THEME.backgroundColor');
    padding: 0vw 0.5vw 0.5vw 0.5vw;
    border-radius: 1vw;
}

/* stuff reorganises on mobile view */
@media screen and (max-width: 900px) {
    div.live-view {
        visibility: hidden;
        display: none;
    }

    div.main {
        flex-flow: column;
        row-gap: 10px;
    }

    div.chart-container {
        flex: 2 2 0;
    }

    div.log-box {
        flex: 1 1 0;
        order: 2;
    }
}
</style>


<style>
body {
    margin: 0;
}

div.body {
    font-family: 'SF Pro', 'Courier New', Courier, monospace;
    background-color: v-bind('THEME.backgroundColor2');
    min-height: 100vh;
    max-height: 100vh;
    clear: both;
    user-select: none;
}


@font-face {
    font-family: "SF Pro";
    font-weight: 400;
    src: url('small-sf-pro.woff2');
}

@font-face {
    font-family: "SF Pro";
    font-weight: 700;
    src: url('small-sf-bold.otf');
}

/* massive screen / TV */
@media screen and (min-width: 3000px) {
    div.header {
        height: 100px !important;
    }

    div.footer {
        height: 40px !important;
    }

    body {
        font-size: 40px !important;
    }
}
</style>

<script setup>
import Header from './components/Header.vue';
import Chart from './components/Chart.vue';
import Footer from './components/Footer.vue';
import Logs from './components/Logs.vue';
import AlertBackground from './components/AlertBackground.vue';
import AlertDisplay from './components/AlertDisplay.vue';
import { THEME } from './theme';
import { ref, computed } from 'vue';
import { packetData, initWebsocket, loaded, calibrations } from './ws.js';

initWebsocket();

const filteredData = computed(() => packetData.filter(t => t != null)
    .filter( // remove 2/3 of points on mobile
        ({ timestamp }) => THEME.isMobile ? (timestamp * 25) % 3 < 1 : 1
    )
    .map(({ pressure, height, timestamp }) => ({
        pressure, height, timestamp: 20 - (Date.now() - timestamp) / 1000
    }))
);


const pressure = computed(() => ({
    values: filteredData.value.map(({ pressure, timestamp }) => ({
        x: timestamp,
        y: pressure
    })),
    loaded: loaded.value
}))

const height = computed(() => ({
    values: filteredData.value.map(({ height, timestamp }) => ({
        x: timestamp,
        y: height
    })),
    loaded: loaded.value,
    baseline: calibrations?.resting_water_level
}))
</script>
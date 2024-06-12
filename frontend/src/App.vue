<!-- This is the main body of the app. It's a bunch of nested flexboxes, yes it's mildly cursed 
    but it's far, far better than the alternative -->
<template>
    <div class="body">
        <div class="header">
            <Header />
        </div>
        <div class="main">
            <div class="alert-container" v-if="THEME.alertActive && THEME.isMobile">
                <AlertDisplay />
            </div>
            <div class="log-box">
                <div class="alert-container" v-if="THEME.alertActive && !THEME.isMobile">
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
                        y: 'Pressure (Pa)',
                        title: 'Sensor Pressure',
                        minY: 1018,
                        maxY: 1022,
                        color: THEME.graphColor2
                    }" :data-source="pressure" />
                </div>
                <div class="live-view chart-box">
                    <Chart name="live-view" :options="{
                        y: 'Wave height (cm)',
                        title: 'Live View',
                        minY: 1018,
                        maxY: 1022,
                        color: THEME.graphColor2
                    }" :data-source="height" />
                </div>
            </div>
        </div>
        <div class="footer">
            <Footer />
        </div>
    </div>
    <Alert />
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
    flex: 2 2;
    flex-basis: 0;
    display: flex;
    flex-direction: column;
    align-items: stretch;
    row-gap: 0.75vw;
}

div.log-container {
    flex: 1 1;
}

div.alert-container {
    flex: 0 0 70px;
}

div.chart-container {
    display: flex;
    flex-flow: column wrap;
    flex: 3 3;
    justify-content: center;
    align-items: stretch;
    row-gap: 1vw;
    flex-basis: 0;
}

div.chart-box {
    flex: 1 1;
    background-color: v-bind('THEME.backgroundColor');
    /*margin: 0.5vw 1vw 0.5vw 0.5vw;*/
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
        flex: 2 2;
        flex-basis: 0;
    }

    div.log-box {
        flex: 1 1;
        flex-basis: 0;
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
    src: url('SF-Pro.ttf');
}
</style>

<script setup>
import Header from './components/Header.vue';
import Chart from './components/Chart.vue';
import Footer from './components/Footer.vue';
import Logs from './components/Logs.vue';
import Alert from './components/Alert.vue';
import AlertDisplay from './components/AlertDisplay.vue';
import { THEME } from './theme';
import { ref, computed } from 'vue';
import { packetData, initWebsocket, loaded } from './ws.js';

initWebsocket();


const pressure = computed(() => ({
    values: packetData.filter(t => t != null)
        .filter(({ height, timestamp }) => THEME.isMobile ? (timestamp * 25) % 2 < 1 : 1) // remove every other point on mobile
        .map(({ pressure, timestamp }) =>
        ({
            x: 20 - (Date.now() - timestamp) / 1000,
            y: pressure
        })),

    loaded: loaded.value
}))

const height = computed(() => ({
    values: packetData.filter(t => t != null)
        .filter(({ height, timestamp }) => THEME.isMobile ? (timestamp * 25) % 3 < 1 : 1) // remove every other point on mobile
        .map(({ height, timestamp }) =>
        ({
            x: 20 - (Date.now() - timestamp) / 1000,
            y: height
        })),
    loaded: loaded.value
}))
</script>
<template>
    <div class="body">
        <Header />
        <!--<div class="padding"></div>-->
        <div class="flex">
            <Chart 
                name="height"
                :options="{
                    y: 'Water level (cm)',
                    title: 'Water Level',
                    minY: 0,
                    maxY: 3,
                    color: THEME.graphColor1
                }"
                :data-source="height"
            />
            <!--<div class="padding"></div>-->
            <Chart 
                name="pressure"
                :options="{
                    y: 'Pressure (Pa)',
                    title: 'Sensor Pressure',
                    minY: 1018,
                    maxY: 1022,
                    color: THEME.graphColor2
                }"
                :data-source="pressure"
            />
            <!--<div class="padding"></div>-->
            <Logs />
            <!--<div class="padding"></div>-->
            <Chart 
                name="live-view"
                :options="{
                    y: 'Wave height (cm)',
                    title: 'Live View',
                    minY: 1018,
                    maxY: 1022,
                    color: THEME.graphColor2
                }"
                :data-source="pressure"
            />
        </div>
        <!--<div class="paddingBottom"></div>-->
        <Footer />
    </div>
</template>

<style scoped>
div.flex {
    display: flex;
    flex-wrap: wrap;
    row-gap: 4vw;
    /*column-gap: 2vw;*/
}

div.padding {
    padding-top: 1vw;
}
/* keeping this typo for posterity */
dev.paddingBottom {

    padding: 100px;
}
/* exept that that code is actually needed so heres it agian without the typo */
div.paddingBottom {

padding: 10px;
}

</style>


<style>
body {
    margin: 0;
}
div.body {
    font-family: "Inter var experimental", "Inter var", Inter, -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Oxygen, Ubuntu, Cantarell, "Fira Sans", "Droid Sans", "Helvetica Neue", sans-serif;
    background-color: v-bind('THEME.backgroundColor');
    min-height: 100vh;
    clear: both;
    user-select: none;
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
        .map(({pressure, timeStamp}) => 
        ({
            x: 20 - (Date.now() - timeStamp) / 1000, 
            y: pressure
        })
    ),
    loaded: loaded.value
}))

const height = computed(() => ({
    values: packetData.filter(t => t != null)
        .map(({waterLevel, timeStamp}) => 
        ({
            x: 20 - (Date.now() - timeStamp) / 1000, 
            y: waterLevel
        })
    ),
    loaded: loaded.value
}))
</script>
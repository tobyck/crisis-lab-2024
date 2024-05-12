<template>
  <Header />
  <div class="padding">
  </div>
  <div class="flex">
        <Chart 
            name="height"
            :options="{
                y: 'Water level (cm)',
                title: 'Water Level',
                minY: 0,
                maxY: 3,
                color: 'skyblue'
            }"
            :data-source="height"
        />
        <div class="padding"></div>
        <Chart 
            name="pressure"
            :options="{
                y: 'Pressure (Pa)',
                title: 'Sensor Pressure',
                minY: 1018,
                maxY: 1022,
                color: 'rgb(100, 126, 255)'
            }"
            :data-source="pressure"
        />
        <div class="padding"></div>
        <div class="padding-in-place-of-log"></div>
        <!-- <Chart 
            name="pressure"
            :options="{
                y: 'Pressure (Pa)',
                title: 'Sensor Pressure',
                minY: 1018,
                maxY: 1022,
                color: 'rgb(100, 126, 255)'
            }"
            :data-source="pressure"
        /> -->
        <div class="padding"></div>
        <Chart 
            name="live-view"
            :options="{
                y: 'Wave height (cm)',
                title: 'Live View',
                minY: 1018,
                maxY: 1022,
                color: 'rgb(100, 126, 255)'
            }"
            :data-source="pressure"
        />
  </div>
  <Footer />
</template>

<style scoped>
div.flex {
    display: flex;
    flex-wrap: wrap;
    row-gap: 1vw;
}
div.padding-in-place-of-log {
    border-radius: 25px;
    border: 2px solid rgb(180, 190, 254);
    width: 39vw;
    margin-left: 5vw;
    margin-right: 5vw;
}

div.padding {
    padding-top: 1vw;
}

</style>

<script setup>
import Header from './components/Header.vue';
import Chart from './components/Chart.vue';
import Footer from './components/Footer.vue';
import { ref, computed } from 'vue';
import { packetData, initWebsocket, loaded } from './ws.js';

initWebsocket();

const pressure = computed(() => ({
    values: packetData.filter(t => t != null)
        .map(({pressure, timeStamp}) => 
        ({
            x: 10 - (Date.now() - timeStamp) / 1000, 
            y: pressure
        })
    ),
    loaded: loaded.value
}))

const height = computed(() => ({
    values: packetData.filter(t => t != null)
        .map(({waterLevel, timeStamp}) => 
        ({
            x: 10 - (Date.now() - timeStamp) / 1000, 
            y: waterLevel
        })
    ),
    loaded: loaded.value
}))
</script>
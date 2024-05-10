<template>
  <Header />
  <div class="flex">
        <Chart 
            name="height"
            :options="{
                y: 'Wave height (cm)',
                title: 'Wave Height',
                minY: 0,
                maxY: 3,
                color: 'skyblue'
            }"
            :data-source="height"
        />
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
  </div>
</template>

<style scoped>
div.flex {
    display: flex;
    flex-wrap: wrap;
}
</style>

<script setup>
import Header from './components/Header.vue';
import Chart from './components/Chart.vue';
import { ref, computed } from 'vue';
import { packetData, initWebsocket, loaded } from './ws.js';

initWebsocket();

const timestamps = computed(() => 
    packetData.map(({timeStamp}) => timeStamp).map(t => {
        let secs = (Date.now() - t) / 1000;
        return secs;
    })
)

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
        .map(({waveHeight, timeStamp}) => 
        ({
            x: 10 - (Date.now() - timeStamp) / 1000, 
            y: waveHeight
        })
    ),
    loaded: loaded.value
}))
</script>
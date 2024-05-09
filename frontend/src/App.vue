<template>
  <Header />
  <div class="flex">
        <Chart 
            name="pressure"
            :data-source="pressure"
        />
  </div>
</template>

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
    //timestamps: timestamps.value, 
    values: packetData.map(({pressure, timeStamp}, ind) => 
        ({
            x: 10 - (Date.now() - timeStamp) / 1000, 
            y: pressure
        })
    ),//.filter(({x,y}, i) => x < 10 && x >= 0.1),
    loaded: loaded.value
}))
</script>
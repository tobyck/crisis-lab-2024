<template>
  <Header />
  <div class="flex">
    <Chart 
      name="pressure"
      :data-source="{pressure}"
    
    />
  </div>
</template>

<script setup>
import Header from './components/Header.vue';
import Chart from './components/Chart.vue';
import { ref } from 'vue';
let pressure = ref(5); // da
</script>
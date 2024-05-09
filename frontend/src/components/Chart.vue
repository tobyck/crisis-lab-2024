<template>
    <div>
        <Line v-if="dataSource.loaded"
            ref="chart"
            :id="name"
            :options="chartOptions"
            :data="chartData"
        />
    </div>
</template>

<style scoped>
div {
    width: 40%;
}
</style>

<script setup>
import { Line } from 'vue-chartjs'
import { ref, computed } from 'vue'
import { Chart as ChartJS, Title, Tooltip, Legend, LineController, LinearScale, CategoryScale, LineElement, PointElement } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, LineController, LinearScale, CategoryScale, LineElement, PointElement)

const props = defineProps(['name','data-source', 'loaded']);
console.log(props.dataSource, props.dataSource.loaded);

const chartData = computed(() => ({
    //labels: props.dataSource.timestamps,
    datasets: [
        {                     
            label: 'Data One',
            backgroundColor: '#f87979',
            pointBackgroundColor: 'white',
            borderColor: 'dodgerblue',
            borderWidth: 1,
            radius: 0,
            pointBorderColor: '#249EBF',
            //Data to be represented on y-axis
            data: props.dataSource.values,
        }
    ] 
}))
const chartOptions = computed(() => ({
    responsive: true,
    scales: {
      x: {
        type: 'linear',
        min: 0,
        max: 10,
        title: {
            text: "Time (s)",
            display: true,
        },
        ticks: {
            // Include a dollar sign in the ticks
            callback(value, index, ticks) {
                return value-10;
            }
        }
      },
      y: {
        min: 1018,
        max: 1022,
        title: {
            text: "Pressure (Pa)",
            display: true,
        },
      }
    },
    animation: {
        duration: 0,
    },
    plugins: {
        legend: {
            display: false,
        },
        title: {
            text: 'Sensor Pressure',
            display: true,
            font: {
                size: 20,
                weight: '',
            }
        }
    }
}))
</script>

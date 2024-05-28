<template>
    <!--<div>-->
    <Line v-if="dataSource.loaded" ref="chart" :id="name" :options="chartOptions" :data="chartData" />
    <!--</div>-->
</template>

<style scoped>
@media screen and (max-width: 900px) {
    div {
        width: 100vw !important;
    }
}
</style>


<script setup>
import { Line } from 'vue-chartjs'
import { ref, computed } from 'vue'
import { THEME } from '@/theme';
import { Chart as ChartJS, Title, Tooltip, Legend, LineController, LinearScale, CategoryScale, LineElement, PointElement } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, LineController, LinearScale, CategoryScale, LineElement, PointElement)

ChartJS.defaults.color = THEME.textColor;
ChartJS.defaults.borderColor = '';
ChartJS.defaults.font.family = "'DejaVu Sans Mono', 'Courier New', Courier, monospace";

const props = defineProps(['name', 'data-source', 'loaded', 'options']);
console.log(props.dataSource, props.dataSource.loaded);


const chartData = computed(() => ({
    //labels: props.dataSource.timestamps,
    datasets: [
        {
            label: 'Data One',
            backgroundColor: '#f87979',
            pointBackgroundColor: 'white',
            borderColor: props.options.color,
            borderWidth: THEME.isMobile ? 1.5 : 3,
            radius: 0,
            pointBorderColor: '#249EBF',
            //Data to be represented on y-axis
            data: props.dataSource.values,
        }
    ]
}))
const chartOptions = computed(() => ({
    responsive: true,
    maintainAspectRatio: false,
    scales: {
        x: {
            type: 'linear',
            min: 0,
            max: 20,
            title: {
                text: "Time (s)",
                display: true,
                color: THEME.textColor,
            },
            ticks: {
                callback(value) {
                    return value - 20;
                },
                color: THEME.textColor,
            },
            grid: {
                color: '',
            },
            border: {
                color: THEME.textColor,
            }
        },
        y: {
            min: props.options.minY,
            max: props.options.maxY,
            title: {
                text: props.options.y,
                display: true,
                color: THEME.textColor,
            },
            ticks: {
                color: THEME.textColor,
            },
            grid: {
                color: '',
            },
            border: {
                color: THEME.textColor,
            }
        }
    },
    normalized: true,
    parsing: false,
    animation: false,
    spanGaps: true,
    animation: false,
    plugins: {
        legend: {
            display: false,
        },
        title: {
            text: props.options.title,
            display: true,
            font: {
                size: 20,
                weight: '',
            },
            color: THEME.textColor,
        },
        tooltip: {
            enabled: false
        }
    }
}))
</script>
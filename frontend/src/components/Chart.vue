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
    border-radius: 25px;
    border-style: solid;
    border-width: 2px;
    border-color: v-bind('THEME.borderColor');
    width: 40vw;
    margin-left: 5vw;
    margin-right: 5vw;
    /*background-color: red; /* temp */
    box-sizing: border-box;
}

@media screen and (max-width: 1200px) {
    div {
        width: max(480px, 40vw);
        margin-left: max(25vw - 240px,0px);
        margin-right: max(25vw - 240px,0px);
    }
}


@media screen and (max-width: 1000px) {
    div {
        width: min(90%, 600px);
        margin-left: max(5%, calc((100% - 600px) / 2));
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
ChartJS.defaults.borderColor = THEME.gridColor;

const props = defineProps(['name','data-source', 'loaded', 'options']);
console.log(props.dataSource, props.dataSource.loaded);


const chartData = computed(() => ({
    //labels: props.dataSource.timestamps,
    datasets: [
        {                     
            label: 'Data One',
            backgroundColor: '#f87979',
            pointBackgroundColor: 'white',
            borderColor: props.options.color,
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
                color: THEME.textColor,
            },
            ticks: {
                callback(value) {
                    return value-10;
                },
                color: THEME.textColor,
            },
            grid: {
                color: THEME.gridColor,
            },
            border: {
                color: THEME.gridColor,
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
                color: THEME.gridColor,
            },
            border: {
                color: THEME.gridColor,
            }
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
            text: props.options.title,
            display: true,
            font: {
                size: 20,
                weight: '',
            },
            color: THEME.textColor,
        }
    }
}))
</script>
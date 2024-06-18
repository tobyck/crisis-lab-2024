<template>
    <Line v-if="dataSource.loaded" ref="chart" :id="name" :options="chartOptions" :data="chartData" />
</template>

<script setup>
import { Line } from 'vue-chartjs'
import { ref, computed, watch } from 'vue'
import { THEME } from '@/theme';
import { Chart as ChartJS, Title, Tooltip, Legend, LineController, LinearScale, CategoryScale, LineElement, PointElement } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, LineController, LinearScale, CategoryScale, LineElement, PointElement)

ChartJS.defaults.color = THEME.textColor;
ChartJS.defaults.borderColor = '';
ChartJS.defaults.font.family = "'SF Pro', 'Courier New', Courier, monospace";
ChartJS.defaults.font.size = window.innerWidth > 3000 ? 20 : 10;

const props = defineProps(['name', 'data-source', 'loaded', 'options']);
console.log(props.dataSource, props.dataSource.loaded);

const borderWidth = computed(() => window.innerWidth > 3000 ? 6 : THEME.isMobile ? 1.5 : 3);

const chartData = computed(() => {
    let res = {
        datasets: [
            {
                label: 'Data One',
                backgroundColor: '#f87979',
                pointBackgroundColor: 'white',
                borderColor: props.options.color,
                borderWidth: borderWidth.value,
                radius: 0,
                pointBorderColor: '#249EBF',
                //Data to be represented on y-axis
                data: props.dataSource.values,
            },
        ]
    }
    // TODO: Threshold, once that's added to the backend
    if (props.dataSource.baseline) {
        res.datasets.push({
            label: 'Threshold',
            backgroundColor: 'rgba(0, 0, 0, 0)',
            borderColor: '#ff0000',
            borderWidth: borderWidth.value,
            borderDash: [5, 5],
            radius: 0,
            data: [{ x: 0, y: props.dataSource.baseline }, { x: 20, y: props.dataSource.baseline }],
        })
    }
    return res;
});

// because vue is stupid this is necessary
let c = computed(() => props.dataSource.values);

let minY = ref(null), maxY = ref(null);

watch(c, (val) => {
    if (minY.value == null) { // uninitialized
        minY.value = Math.min(...val.map(v => v.y));
        maxY.value = Math.max(...val.map(v => v.y));
    } else {
        let mostRecent = (val.at(-1) ?? val.findLastIndex(x => x)).y;
        // I would use Math.min/max but it triggers a rescale constantly
        if (mostRecent < minY.value) minY.value = mostRecent;
        if (mostRecent > maxY.value) maxY.value = mostRecent;
    }
});

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
                color: THEME.backgroundColor2,
            },
            border: {
                color: THEME.textColor,
            }
        },
        y: {
            // this is a mess
            min: Math.floor((minY.value - (maxY.value - minY.value) * 0.15) * 10) / 10,
            max: Math.ceil((maxY.value + (maxY.value - minY.value) * 0.15) * 10) / 10,
            title: {
                text: props.options.y,
                display: true,
                color: THEME.textColor,
            },
            ticks: {
                color: THEME.textColor,
            },
            grid: {
                color: THEME.backgroundColor2,
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
                size: window.innerWidth > 3000 ? 40 : 20,
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
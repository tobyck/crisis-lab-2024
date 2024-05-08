<template>
    <div>
        <Line
            :id="props.name"
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
const props = defineProps(['name','data-source']);
console.log(props.dataSource);
</script>

<script>
import { Line } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, LineController, LinearScale, CategoryScale, LineElement, PointElement } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, LineController, LinearScale, CategoryScale, LineElement, PointElement )

export default {
  name: 'BarChart',
  components: { Line },
  data() {
    return {
      chartData: {
        labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July', 'August', 'September', 'October', 'November', 'December'],
        datasets: [ {                     label: 'Data One',
                    backgroundColor: '#f87979',
                    pointBackgroundColor: 'white',
                    borderWidth: 1,
                    pointBorderColor: '#249EBF',
                    //Data to be represented on y-axis
                    data: [40, 20, 30, 50, 90, 10, 20, 40, 50, 70, 90, 100] } ]
      },
      chartOptions: {
        responsive: true
      }
    }
  }
}


</script>
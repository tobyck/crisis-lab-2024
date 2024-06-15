<!--just a little blinky light-->
<template>
    <span class="container">
        {{ props.name }}
        <span v-if="status" class="light-on">
            online
        </span>
        <span v-else class="light-off">
            offline
        </span>
    </span>
</template>

<script setup>
const props = defineProps(['name', 'status']);
import { THEME } from '../theme.js';
</script>

<style scoped>
.light-on {
    color: v-bind('THEME.statusLightOn');
}

.light-off {
    color: v-bind('THEME.statusLightOff');
}

span.container {
    padding-top: 1vh;
    font-size: 10px;
    margin-right: 20px;
    font-weight: 700;
}
</style>
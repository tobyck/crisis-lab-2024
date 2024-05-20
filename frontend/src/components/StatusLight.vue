
<template>
    <span class="container">
        {{ props.name }}
        <span v-if="status">
            Online <span class="status-light light-on"></span>
        </span>
        <span v-else>
            Offline <span class="status-light light-off"></span>
        </span>
    </span>
</template>

<script setup>
const props = defineProps(['name','status']);
import { THEME } from '../theme.js';
</script>

<style scoped>
.status-light {
    margin: 0px 0px;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    /*float: left;*/
    display: inline-block;
}
.light-on{
    background-color: v-bind('THEME.statusLightOn');
}

.light-off {
    background-color: v-bind('THEME.statusLightOff');
}

span.container {
    font-size: 10px;
    float: right;
    margin-right: 20px;
}
</style>
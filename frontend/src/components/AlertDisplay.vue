<!-- this pops up if an alert occurs -->
<template>
    <div class="box">
        <div class="exc">⚠️</div>
        <div class="rest">
            A tsunami of height {{ logs[0].match(/[\d.]+cm/)[0] }} is occuring. <br>
            Evacuate to higher ground immediately.
        </div>
    </div>
</template>

<style scoped>
.box {
    border-radius: 1vw;
    height: 50px;
    background-color: v-bind('THEME.backgroundColor');
    padding: 1.3vw;
    text-align: center;
    border: 3px solid v-bind('THEME.dark ? "#aa0000" : "#ff000077"');
}

.exc {
    font-size: 42px;
    display: inline-block;
    padding-right: 15px;
}

.rest {
    font-size: 20px;
    display: inline-block;
    color: v-bind('THEME.textColor');
    /*width: calc(100% - 50px);*/
}
</style>

<script setup>
import { THEME } from '@/theme.js'
import { logs } from '@/ws.js'
</script>
<!-- This is just a semitransparent red flashing overlay for when alerts appear -->
<template>
    <div class="alert-bg" v-if="THEME.alertActive"></div>
</template>

<style scoped>
.alert-bg {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: red;
    z-index: 100;
    display: flex;
    justify-content: center;
    align-items: center;
    color: v-bind('THEME.textColor');
    /*font-size: ;*/
    animation: blink 1s infinite;
    user-select: none;
    pointer-events: none;
}

@keyframes blink {
    from {
        opacity: v-bind('THEME.dark ? .1 : .2')
    }

    50% {
        opacity: v-bind('THEME.dark ? .2 : .3')
    }

    to {
        opacity: v-bind('THEME.dark ? .1 : .2')
    }
}
</style>


<script setup>
import { THEME } from '../theme.js';
</script>
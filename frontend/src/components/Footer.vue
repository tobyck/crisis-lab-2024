<script setup>
import { THEME } from '../theme.js';
</script>

<template>
    <div>
        <a href='https://github.com/tobyck/crisis-lab-2024' target='_blank'>GitHub</a>
        <a href='https://www.instagram.com/crisislabs.2024.whs/' target='_blank'>Instagram</a>
        <a href='javascript:;' @click='mailClicked'>Mailing list</a>
        <DarkToggle />
    </div>
</template>

<style scoped>
a {
    color: v-bind('THEME.textColor');
    padding: 5px;
    text-decoration: none;
    /*margin-right: 10px;*/
}
a:hover {
    text-decoration:underline;
}
div {
    position: fixed;
    width: 100%;
    bottom: 0;
    background-color: v-bind('THEME.lighterBackground');
    padding: 0.4vh;
    /*text-align: center;*/
}

</style>

<script>
import { THEME } from '../theme.js';
import DarkToggle from './DarkToggle.vue';
function mailClicked() {
    console.log('todo: open mail dialog')
}
</script>
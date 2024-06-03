<script setup>
import { THEME } from '../theme.js';
</script>

<template>
    <div class="footer">
        <span class="links">
            <a href='https://github.com/tobyck/crisis-lab-2024' target='_blank'>GitHub</a>
            <a href='https://www.instagram.com/crisislabs.2024.whs/' target='_blank'>Instagram</a>
            <a href='javascript:;' @click='mailClicked'>Mailing list</a>
        </span>
        <DarkToggle />
    </div>
</template>

<style scoped>
a {
    color: v-bind('THEME.textColor');
    text-decoration: none;
    margin-left: 10px;
    margin-right: 10px;
    user-select: all !important;
}

a:hover {
    text-decoration: underline;
}

div {
    position: fixed;
    width: 100%;
    bottom: 0;
    background-color: v-bind('THEME.backgroundColor2');
    padding: 0.4vh;
    z-index: 100;
    /*text-align: center;*/
}

.links {
    line-height: 18px;
}
</style>

<style>
@media screen and (max-width: 460px) {
    div.footer {
        font-size: 10px !important;
    }
}
</style>

<script>
import { THEME } from '../theme.js';
import DarkToggle from './DarkToggle.vue';
function mailClicked() {
    console.log('todo: open mail dialog')
}
</script>
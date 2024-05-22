<script setup>
import { THEME } from '../theme.js';
</script>

<template>
    <header>
        <span class="textTsunami" v-if="THEME.alertActive">&#128680; A TSUNAMI IS HAPPENING &#128680;</span>
        <span class="textNormal" v-if="!THEME.alertActive">CRISiSLab 2024 Dashboard</span>
    </header>
</template>

<style scoped>
header>span.textNormal {
    height: 50px;
    font-size: 40px;
    background: -webkit-linear-gradient(315deg, v-bind('THEME.headerColor1'), v-bind('THEME.headerColor2'));
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;
}

header {
    font-size: 40px;
    text-align: center;
    /*border-radius: 25px;
    border: 2px solid v-bind('THEME.borderColor');*/
    margin-left: max(25vw - 240px,0px);
    margin-right: max(25vw - 240px,0px);
    padding-top: 1vw;
    padding-bottom: 1vw;
}

@media screen and (min-width: 2400px) {
    header>span {
        font-size: 120px;
    }
}

span.textTsunami {
    color: v-bind('THEME.borderColor');
}
</style>
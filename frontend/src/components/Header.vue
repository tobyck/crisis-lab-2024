<script setup>
import { THEME } from '../theme.js';
</script>

<template>
    <header>
        <!--<span class="textTsunami" v-if="THEME.alertActive">&#128680; A TSUNAMI IS HAPPENING &#128680;</span>-->
        <span class="textNormal">CRISiSLab 2024 Dashboard</span>
    </header>
</template>

<style scoped>
header>span.textNormal {
    height: 50px;
    font-size: 40px;
    /*background: -webkit-linear-gradient(315deg, v-bind('THEME.headerColor1'), v-bind('THEME.headerColor2'));
    -webkit-background-clip: text;
    background-clip: text;
    -webkit-text-fill-color: transparent;*/
    color: v-bind('THEME.textColor');
    font-weight: 700;
}

header {
    font-size: 40px;
    text-align: center;
    width: 100vw;
    padding-top: 0.5vw;
    /*background-color: v-bind('THEME.backgroundColor2');*/
}

@media screen and (max-width: 550px) {
    header>span {
        font-size: 25px !important;
    }
}

span.textTsunami {
    color: v-bind('THEME.borderColor');
}
</style>
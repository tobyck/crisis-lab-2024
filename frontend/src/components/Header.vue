<script setup>
import { THEME } from '../theme.js';
</script>

<template>
    <header>
        <span class="textNormal">CRISiSLab 2024 Dashboard</span>
    </header>
</template>

<style scoped>
header>span.textNormal {
    height: 50px;
    font-size: 40px;
    color: v-bind('THEME.textColor');
    font-weight: 700;
}

header {
    text-align: center;
    width: 100vw;
    padding-top: 0.5vw;
}

/* massive screen / TV */
@media screen and (min-width: 3000px) {
    header>span.textNormal {
        font-size: 90px !important;
    }
}

@media screen and (max-width: 550px) {
    header>span {
        font-size: 25px !important;
    }
}
</style>
<!-- this pops up if an alert occurs -->
<!-- for complicated reasons we have two of these - one for mobile, one otherwise -->
<template>
    <div class="box">
        <div v-if="THEME.alertActive" class="exc">⚠️</div>
        <div v-if="THEME.alertActive" class="rest">
            A tsunami of height {{ logs[0].match(/[\d.]+cm/)[0] }} is occuring. <br>
            Evacuate to higher ground immediately.
        </div>
        <div v-if="!THEME.alertActive" class="rest">
            If an earthquake is <b>Long OR Strong, Get Gone!</b><br>
            <span class="smaller">Click <a href="https://www.wremo.nz/hazards/tsunami" target="_blank">here</a>
                for more information about tsunami in your area.
            </span>
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
    border: 3px solid v-bind('THEME.alertActive ? THEME.dark ? "#aa0000" : "#ff000077" : "transparent"');
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
}

.smaller {
    font-size: 15px;
}

a {
    color: v-bind('THEME.textColor');
}

@media screen and (min-width: 3000px) {
    .exc {
        font-size: 90px;
    }

    .rest {
        font-size: 40px;
    }

    .smaller {
        font-size: 30px;
    }

    .box {
        height: 100px;
        border-width: 6px;
    }
}

@media screen and (max-width: 550px) {
    .rest {
        font-size: 15px;
    }

    .smaller {
        font-size: 12px;
    }
}
</style>

<script setup>
import { THEME } from '@/theme.js'
import { logs } from '@/ws.js'
</script>
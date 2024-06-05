<template>
    <span class="box">
        <span class="text">
            Dark Mode
        </span>
        <label class="switch" for="checkbox">
            <input type="checkbox" id="checkbox" @click="THEME.toggleDark()"
                :checked="THEME.defaultMode ? 'true' : undefined">
            <span class="slider round"></span>
        </label>
    </span>
</template>
<style scoped>
.switch {
    position: relative;
    display: inline-block;
    width: 30px;
    height: 18px;
}


.box {
    float: right;
    padding-right: 10px;
    color: v-bind('THEME.textColor');
    line-height: 18px;
}

@media screen and (max-width: 460px) {

    label {
        margin-top: 0px !important;
    }
}

.switch input {
    opacity: 0;
    width: 0;
    height: 0;
}

.slider {
    position: absolute;
    cursor: pointer;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    border: 1px solid v-bind('THEME.textColor');
    -webkit-transition: .4s;
    transition: .4s;
    border-radius: 20px;
}

.slider:before {
    position: absolute;
    content: "";
    height: 12px;
    width: 12px;
    left: 2px;
    bottom: 2px;
    background-color: v-bind('THEME.backgroundColor');
    -webkit-transition: .4s;
    transition: .4s;
    border-radius: 50%;
    font-size: 10px;
    color: v-bind('THEME.textColor');
    content: '☼';
    text-align: center;
    line-height: 11.7px;
    font-family: 'Arial', sans-serif;
}

input:checked+.slider:before {
    -webkit-transform: translateX(12px);
    -ms-transform: translateX(12px);
    transform: translateX(12px);
    content: "☾";
    /* I apologize for the hair space, but it's necessary */
}

/* Rounded sliders */
.slider.round {
    border-radius: 18px;
}

.slider.round:before {
    border-radius: 50%;
}

label {
    margin-top: 2px;
}
</style>

<script setup>
import { THEME } from '../theme.js'
</script>
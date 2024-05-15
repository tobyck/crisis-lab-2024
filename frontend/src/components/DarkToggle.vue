<template>
    <label class="switch" for="checkbox">
        <input type="checkbox" id="checkbox" @click="toggle()">
        <span class="slider round"></span>
    </label>
</template>
<style scoped>
.switch {
    position: relative;
    display: inline-block;
    width: 30px;
    height: 18px;
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
    /*background-color: #ccc;*/
    border: 1px solid v-bind('THEME.borderColor');
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
    background-color: white;
    -webkit-transition: .4s;
    transition: .4s;
    border-radius: 50%;
}

input:checked + .slider {
  /*background-color: #2196F3;*/
}

input:focus + .slider {
  box-shadow: 0 0 1px #2196F3;
}

input:checked + .slider:before {
  -webkit-transform: translateX(12px);
  -ms-transform: translateX(12px);
  transform: translateX(12px);
}

/* Rounded sliders */
.slider.round {
  border-radius: 18px;
}

.slider.round:before {
  border-radius: 50%;
}
</style>

<script setup>
import { THEME } from '../theme.js'
let toggle = () => THEME.dark = !THEME.dark
</script>
<template>
    <label class="switch" for="checkbox">
        <input type="checkbox" id="checkbox" @click="toggle()" :checked="THEME.defaultMode ? 'true' : undefined">
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
    background-color: v-bind('THEME.lighterBackground');
    -webkit-transition: .4s;
    transition: .4s;
    border-radius: 50%;
    font-size: 10px;
    color: v-bind('THEME.textColor');
    content: ' ☼'
}

input:checked + .slider:before {
  -webkit-transform: translateX(12px);
  -ms-transform: translateX(12px);
  transform: translateX(12px);
  content: "  ☾"; /* I apologize for the hair space, but it's necessary */
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
let toggle = () => localStorage.setItem('dark',THEME.dark = !THEME.dark)
</script>
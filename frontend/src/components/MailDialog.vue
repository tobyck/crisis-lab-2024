<template>
    <div class="dialog-background">
        <div class="dialog">
            Please enter your email address<br> to join the mailing list:<br>
            <input type="email" v-model="email" class="email" />
            <button class="subscribe" @click="subscribeEmail(email)">Subscribe</button>
            <div class="error">{{ error }}</div>
            <div class="close" @click="$emit('close')">&times;</div>
        </div>
    </div>
</template>

<style scoped>
.dialog-background {
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background-color: rgba(0, 0, 0, 0.4);
    z-index: 100;
    display: flex;
    justify-content: center;
    align-items: center;
    color: v-bind('THEME.textColor');
}

.dialog {
    background-color: v-bind('THEME.backgroundColor2');
    padding: 1em;
    border-radius: 1em;
    width: 260px;
    height: 400px;
    text-align: center;
    position: relative;
}

.close {
    position: absolute;
    top: 0;
    right: 0;
    padding: 0.5em;
    width: 20px;
    height: 20px;
    cursor: pointer;
}

input.email {
    width: 80%;
    margin: 1em;
    padding: 0.5em;
    border-radius: 0.5em;
    border: 1px solid v-bind('THEME.textColor');
}

.subscribe {
    margin: 0.5em;
    padding: 0.5em;
    border-radius: 0.5em;
    border: 1px solid v-bind('THEME.textColor');
    background-color: v-bind('THEME.backgroundColor');
    color: v-bind('THEME.textColor');
    cursor: pointer;
}

.error {
    color: red;
    font-size: 0.8em;
}
</style>

<script setup>
import { ref } from 'vue';
import { THEME } from '@/theme';

let error = ref('');

const EMAILREGEX = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;

let subscribeEmail = (email) => {
    if (!EMAILREGEX.test(email)) {
        error.value = 'Invalid email address';
        return;
    } else {
        fetch('https://dashboard.alex-berry.net:8783/subscribe?email=' + encodeURI(email)).then(res => {
            if (res.status === 200) {
                error.value = 'Subscribed!';
            } else {
                error.value = 'Error subscribing';
            }
        }).catch(err => {
            error.value = 'Error subscribing';
        });
    }
}


</script>
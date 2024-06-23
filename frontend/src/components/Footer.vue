<script setup>
import { THEME } from '../theme.js';
</script>

<template>
    <div class="footer">
        <span class="links">
            <a href='https://github.com/tobyck/crisis-lab-2024' target='_blank'>GitHub</a>
            <a href='https://www.instagram.com/crisislabs.2024.whs/' target='_blank'>Instagram</a>
            <a href='javascript:;' @click='openMail'>Mailing list</a>
            <a href='https:/discord.gg/39mn5Zhshn' target='_blank'>Discord</a>
        </span>
        <DarkToggle />
    </div>
    <MailDialog v-if='mailOpen' @close="closeMail" />
</template>

<style scoped>
a {
    color: v-bind('THEME.textColor');
    text-decoration: none;
    margin-left: 0.5vw;
    margin-right: 0.5vw;
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
    padding: 0.5vw;
    z-index: 100;
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
import { ref } from 'vue';
import MailDialog from './MailDialog.vue';

const mailOpen = ref(false);

let openMail = () => mailOpen.value = true;
let closeMail = () => mailOpen.value = false;
</script>
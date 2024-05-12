<template>
    <div>
        <a href='https://github.com/tobyck/crisis-lab-2024' target='_blank'>GitHub</a>
        <a href='https://www.instagram.com/crisislabs.2024.whs/' target='_blank'>Instagram</a>
        <a href='javascript:;' @click='mailClicked'>Mailing list</a>
    </div>
</template>

<style scoped>
a {
    color: #a6adc8;
    padding: 5px;
    text-decoration: none;
}
a:hover {
    text-decoration:underline;
}
div {
    position: fixed;
    width: 100%;
    bottom: 0;
    background-color: var(--backgroundColor);
}
</style>

<script setup>
function mailClicked() {
    console.log('todo: open mail dialog')
}
</script>
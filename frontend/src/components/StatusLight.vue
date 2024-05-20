<template>
    <span class="container">
        {{ props.name }}
        <span v-if="status">
            Online <span class="status-light light-on"></span>
        </span>
        <span v-else>
            Offline <span class="status-light light-off"></span>
        </span>
    </span>
</template>

<script setup>
const props = defineProps(['name','status']);

</script>

<style scoped>
.status-light {
    margin: 0px 0px;
    width: 8px;
    height: 8px;
    border-radius: 50%;
    /*float: left;*/
    display: inline-block;
}
.light-on{
    background-color: rgb(141, 212, 0);
    box-shadow: #225 0 -1px 7px 1px, inset rgb(90, 135, 0) 0 -1px 9px, #7D0 0 0px 8px;
}

.light-off {
    background-color: #940;
    box-shadow: #225 0 -1px 7px 1px, inset #600 0 -1px 9px, #F00 0 0px 8px;
}

span.container {
    font-size: 10px;
    float: right;
    margin-right: 20px;
}
</style>
<script setup lang="ts">
const store = useEditorStore()
await callOnce(store.fetchSchema)
</script>

<template>
    <NuxtRouteAnnouncer/>
    <NuxtLayout>
        <NuxtPage/>
    </NuxtLayout>
</template>

<style>
@import '@vue-flow/core/dist/style.css';
@import '@vue-flow/core/dist/theme-default.css';
</style>

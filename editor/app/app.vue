<script setup lang="ts">
const store = useEditorStore()
await callOnce(store.fetchSchema)
</script>

<template>
    <NuxtRouteAnnouncer/>
    <div class="font-default text-text-100">
        <NuxtLayout>
            <NuxtPage/>
        </NuxtLayout>
    </div>
</template>

<style>
@import '@vue-flow/core/dist/style.css';
@import '@vue-flow/core/dist/theme-default.css';
</style>

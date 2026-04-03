<script setup lang="ts">
import VismutHeader from "~/components/editor/VismutHeader.vue";
import VismutNodeExplorer from "~/components/editor/VismutNodeExplorer.vue";
import VismutFileExplorer from "~/components/editor/VismutFileExplorer.vue";
</script>

<template>
    <header class="fixed left-0 right-0 p-4 z-3">
        <VismutHeader/>
    </header>
    <slot/>
    <div class="fixed flex gap-4 left-4 top-24 bottom-4 right-4 items-end pointer-events-none">
        <VismutNodeExplorer class="pointer-events-auto not-lg:hidden"/>
        <VismutFileExplorer class="pointer-events-auto w-full"/>
    </div>

</template>

<style scoped>

</style>
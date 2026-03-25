<script lang="ts" setup>
defineProps(["data", "start", "end", "path"])
</script>

<template>
    <svg class="overflow-visible absolute pointer-events-none w-2.5 h-2.5" data-testid="connection">
        <path :d="path" class="fill-none stroke-[5px] stroke-text-100 pointer-events-auto rounded-full"/>
    </svg>
</template>

<style scoped>

</style>
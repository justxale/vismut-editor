<script setup lang="ts">
defineProps<{
    title: string
}>()
</script>

<template>
    <div class="bg-panel-300 rounded-2xl min-h-32 max-h-80 min-w-32 max-w-32 pb-1.5 select-none box-border flex-1">
        {{ title }}
    </div>
</template>

<style scoped>

</style>
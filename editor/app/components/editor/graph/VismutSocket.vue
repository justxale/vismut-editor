<script lang="ts" setup>
import {ClassicPreset} from "rete";

defineProps<{
    data: ClassicPreset.Socket
}>()
</script>

<template>
    <div
        :title="data.name"
        class="inline-block cursor-pointer rounded-full w-5 h-5 align-middle bg-vismut-red hover:border-3 border-text-100"
    />
</template>

<style scoped>

</style>
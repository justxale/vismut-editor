<script setup lang="ts">
defineProps<{data: NodeSchema}>()
</script>

<template>
<div class="bg-panel-200 rounded-2xl min-h-32 max-h-80 min-w-32 grow pb-1.5 select-none box-border cursor-pointer text-text-100">
    <div class="text-white text-sm p-2 bg-vismut-red rounded-t-2xl mb-2" data-testid="title">{{ data.node_id }}</div>
    <div class="w-full grid grid-cols-2 px-2">
        <div>
            <div v-for="input in data.inputs">
                <div class="inline-block rounded-full w-4 h-4 align-middle bg-vismut-red hover:border-3 border-text-100"></div>
                {{ input.name }} <code>{{ input.types.toString() }}</code>
            </div>
        </div>
        <div class="flex flex-col items-end">
            <div v-for="output in data.outputs" class="">
                <code>{{ output.types.toString() }}</code> {{ output.name }}
                <div class="inline-block rounded-full w-4 h-4 align-middle bg-vismut-red"></div>
            </div>
        </div>
    </div>
</div>
</template>

<style scoped>

</style>
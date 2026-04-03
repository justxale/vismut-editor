<script setup lang="ts">
import ComponentPanel from "~/components/ComponentPanel.vue"
import NodeDisplay from "~/components/editor/displays/NodeDisplay.vue"

const isVisible = ref(true);
const store = useEditorStore()
const { onDragStart } = useDragAndDrop()
</script>

<template>
    <div class="flex items-start gap-2 h-full relative">
        <ComponentPanel v-if="isVisible" class="p-4 w-[25vw] h-full">
            <div v-if="store.schema" class="grid grid-cols-[repeat(auto-fill,minmax(192px,4fr))] grid-rows-[repeat(auto-fill,minmax(128px,4fr))] gap-4 overflow-y-auto h-full">
                <NodeDisplay v-for="node in store.schema.nodes" :data="node" draggable="true" @dragstart="onDragStart"/>
            </div>
        </ComponentPanel>
        <button @click="isVisible = !isVisible" class="text-text-100 absolute -right-12">Close</button>
    </div>
</template>

<style scoped>
</style>
<script setup lang="ts">
import {ConnectionMode, useVueFlow, VueFlow} from "@vue-flow/core";
import {Background} from "@vue-flow/background"
import VismutNode from "~/components/editor/graph/VismutNode.vue";

const { addEdges, onConnect, updateEdge, onEdgeUpdate, removeEdges, onEdgeUpdateEnd } = useVueFlow()
const { onDragOver, onDrop, onDragLeave, isDragOver } = useDragAndDrop()

const nodes = ref([
    {
        id: '1',
        type: 'vismut',
        position: { x: 250, y: 5 },
        data: { label: 'Node 1' },
    },
    {
        id: '2',
        type: 'vismut',
        position: { x: 100, y: 100 },
        data: { label: 'Node 2' },
    },
    {
        id: '3',
        type: 'vismut',
        position: { x: 400, y: 200 },
        data: { label: 'Node 3' },
    }
])
const edges = ref([
    {
        id: 'e1->2',
        source: '1',
        target: '2',
    }
])
const updateState = ref<boolean>(false)

onConnect((connection) => {
    addEdges(connection)
})

onEdgeUpdate(({edge, connection}) => {
    updateEdge(edge, connection)
    updateState.value = true
})

onEdgeUpdateEnd(({edge}) => {
    if (!updateState.value) {
        removeEdges(edge)
    }
    updateState.value = false
})
</script>

<template>
<div class="fixed top-0 right-0 left-0 bottom-0 bg-panel-100" @drop="onDrop">
    <ClientOnly>
        <VueFlow
            :nodes="nodes" :edges="edges" :connection-mode="ConnectionMode.Strict"
            apply-default fit-view-on-init edges-updatable
            @dragover="onDragOver" @dragleave="onDragLeave"
        >
            <Background/>
            <template #node-vismut="props">
                <VismutNode v-bind="props"/>
            </template>
        </VueFlow>
    </ClientOnly>
</div>
</template>

<style scoped>

</style>
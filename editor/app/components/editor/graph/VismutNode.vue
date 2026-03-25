<script lang="ts" setup>
import {Ref,} from 'rete-vue-plugin'
import {type ClassicPreset} from "rete";

const props = defineProps<{
    data: ClassicPreset.Node,
    emit: any,
    seed: number
}>()

function sortByIndex(entries: Record<string, any>) {
    entries.sort((a: { index: number }[], b: { index: number }[]) => {
        const ai = a[1] && a[1].index || 0
        const bi = b[1] && b[1].index || 0

        return ai - bi
    })
    return entries
}

const inputs = computed(() => sortByIndex(Object.entries(props.data.inputs)))
const controls = computed(() => sortByIndex(Object.entries(props.data.controls)))
const outputs = computed(() => sortByIndex(Object.entries(props.data.outputs)))
</script>

<template>
    <div class="bg-panel-200 rounded-2xl cursor-pointer h-auto pb-1.5 select-none box-border">
        <div class="text-white text-sm p-2 bg-vismut-red rounded-t-2xl mb-2" data-testid="title">{{ data.label }}</div>
        <div v-for="[key, output] in outputs" :key="key + seed" :data-testid="'output-' + key" class="text-right">
            <div class="align-middle text-sm inline-block text-white m-4 " data-testid="output-title">
                {{ output.label }}
            </div>
            <Ref
                :data="{
                    type: 'socket',
                    side: 'output',
                    key: key,
                    nodeId: data.id,
                    payload: output.socket
                }"
                :emit="emit"
                class="-mr-px inline-block text-right relative left-2"
                data-testid="output-socket"
            />
        </div>
        <Ref
            v-for="[key, control] in controls"
            :key="key + seed"
            :data="{type: 'control', payload: control}"
            :data-testid="'control-' + key"
            :emit="emit"
            class="px-16 py-2"
        />
        <div
            v-for="[key, input] in inputs"
            :key="key + seed"
            :data-testid="'input-' + key"
            class="text-left"
        >
            <Ref
                :data="{
                    type: 'socket',
                    side: 'input',
                    key: key,
                    nodeId: data.id,
                    payload: input.socket,
                }"
                :emit="emit"
                class="-mr-px inline-block text-left relative right-2"
                data-testid="input-socket"
            />
            <div
                v-show="!input.control || !input.showControl"
                class="align-middle text-sm inline-block text-white m-4"
                data-testid="input-title"
            >
                {{ input.label }}
            </div>
            <Ref
                v-show="input.control && input.showControl"
                :data="{ type: 'control', payload: input.control }"
                :emit="emit"
                class="z-10 w-[70%] align-middle inline-block"
                data-testid="input-control"
            />
        </div>
    </div>
</template>

<style scoped>

</style>
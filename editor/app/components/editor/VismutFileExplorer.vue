<script setup lang="ts">
import ComponentPanel from "~/components/ComponentPanel.vue"
import FileDisplay from "~/components/editor/displays/FileDisplay.vue"

const isVisible = ref<boolean>(true)
</script>

<template>
    <div>
        <button @click="isVisible = !isVisible" class="text-text-100">Close</button>
        <ComponentPanel v-if="isVisible" class="p-4 h-[25vh] p-auto">
            <div class="grid grid-cols-[repeat(auto-fill,minmax(128px,4fr))] gap-4 overflow-y-auto h-full">
                <FileDisplay v-for="i in 20" :title="'File' + i.toString()"/>
            </div>
        </ComponentPanel>
    </div>
</template>

<style scoped>

</style>
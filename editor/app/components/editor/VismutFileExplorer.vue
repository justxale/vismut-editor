<script setup lang="ts">
import ComponentPanel from "~/components/ComponentPanel.vue"
import FileDisplay from "~/components/editor/displays/FileDisplay.vue"
import {Tabs, TabsContent, TabsList, TabsTrigger} from "~/components/ui/tabs";

const isVisible = ref<boolean>(true)
</script>

<template>
    <div>
        <button @click="isVisible = !isVisible" class="text-text-100">Close</button>
        <Tabs default-value="console" v-if="isVisible">
            <TabsList>
                <TabsTrigger value="console">
                    Console
                </TabsTrigger>
                <TabsTrigger value="files">
                    Files
                </TabsTrigger>
            </TabsList>
            <TabsContent value="console">
                <ComponentPanel class="p-4 h-[25vh] p-auto">

                </ComponentPanel>
            </TabsContent>
            <TabsContent value="files">
                <ComponentPanel class="p-4 h-[25vh] p-auto">
                    <div class="grid grid-cols-[repeat(auto-fill,minmax(128px,4fr))] gap-4 overflow-y-auto h-full">
                        <FileDisplay v-for="i in 20" :title="'File' + i.toString()"/>
                    </div>
                </ComponentPanel>
            </TabsContent>
        </Tabs>
    </div>
</template>

<style scoped>

</style>
<script setup lang="ts">
import {Menubar, MenubarContent, MenubarTrigger, MenubarItem, MenubarMenu} from "~/components/ui/menubar"
import ComponentPanel from "~/components/ComponentPanel.vue"
</script>

<template>
    <ComponentPanel class="px-4 py-2">
        <Menubar>
            <NuxtImg src="/vismut.svg" width="24" class="mr-2"/>
            <MenubarMenu value="file">
                <MenubarTrigger value="file">File</MenubarTrigger>
                <MenubarContent>
                    <MenubarItem>Create</MenubarItem>
                    <MenubarItem>Open</MenubarItem>
                </MenubarContent>
            </MenubarMenu>
            <MenubarMenu value="edit">
                <MenubarTrigger value="edit">Edit</MenubarTrigger>
                <MenubarContent>
                    <MenubarItem>Add Node</MenubarItem>
                </MenubarContent>
            </MenubarMenu>
        </Menubar>
    </ComponentPanel>
</template>

<style scoped>

</style>
<script setup lang="ts">
import {Menubar, MenubarContent, MenubarTrigger, MenubarItem, MenubarMenu} from "~/components/ui/menubar";
</script>

<template>
<div class="fixed left-0 right-0 p-4 z-3">
    <header class="bg-panel-300/60 rounded-full w-full backdrop-blur-sm border border-text-100/20 px-4 py-2">
        <Menubar>
            <NuxtImg src="/vismut.svg" width="24" class="mr-2"/>
            <MenubarMenu value="file">
                <MenubarTrigger value="file">File</MenubarTrigger>
                <MenubarContent>
                    <MenubarItem>Create</MenubarItem>
                    <MenubarItem>Open</MenubarItem>
                </MenubarContent>
            </MenubarMenu>
        </Menubar>
    </header>
</div>
</template>

<style scoped>

</style>
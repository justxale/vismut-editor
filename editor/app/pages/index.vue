<script setup lang="ts">
const reteRef = useTemplateRef("reteRef")

onMounted(async () => {
    await createEditor(reteRef.value!)
})
</script>

<template>
<div ref="reteRef" class="w-screen min-h-screen bg-panel-100"/>
</template>

<style scoped>

</style>
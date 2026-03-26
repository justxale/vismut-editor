<script setup lang="ts">
const reteRef = useTemplateRef("reteRef")

onMounted(async () => {
    await createEditor(reteRef.value!)
})
</script>

<template>
<div ref="reteRef" class="fixed top-0 right-0 left-0 bottom-0 bg-panel-100"/>
</template>

<style scoped>

</style>
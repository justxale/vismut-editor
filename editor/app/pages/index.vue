<script setup lang="ts">
const ref = useTemplateRef("rete")

onMounted(async () => {
    await createEditor(ref.value!)
})
</script>

<template>
<div ref="rete" class="w-screen min-h-screen">

</div>
</template>

<style scoped>

</style>
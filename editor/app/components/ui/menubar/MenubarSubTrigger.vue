<script setup lang="ts">
import type { MenubarSubTriggerProps } from "reka-ui"
import type { HTMLAttributes } from "vue"
import { reactiveOmit } from "@vueuse/core"
import { ChevronRight } from "lucide-vue-next"
import { MenubarSubTrigger, useForwardProps } from "reka-ui"
import { cn } from "@/utils/utils"

const props = defineProps<MenubarSubTriggerProps & { class?: HTMLAttributes["class"], inset?: boolean }>()

const delegatedProps = reactiveOmit(props, "class", "inset")
const forwardedProps = useForwardProps(delegatedProps)
</script>

<template>
  <MenubarSubTrigger
    data-slot="menubar-sub-trigger"
    :data-inset="inset ? '' : undefined"
    v-bind="forwardedProps"
    :class="cn(
      'focus:bg-panel-300 focus:text-text-100 data-[state=open]:bg-panel-300 data-[state=open]:text-text-100 flex cursor-default items-center rounded-sm px-2 py-1.5 text-sm outline-none select-none data-[inset]:pl-8',
      props.class,
    )"
  >
    <slot />
    <ChevronRight class="ml-auto size-4" />
  </MenubarSubTrigger>
</template>

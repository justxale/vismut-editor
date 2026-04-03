<script setup lang="ts">
import type { MenubarTriggerProps } from "reka-ui"
import type { HTMLAttributes } from "vue"
import { reactiveOmit } from "@vueuse/core"
import { MenubarTrigger, useForwardProps } from "reka-ui"
import { cn } from "@/utils/utils"

const props = defineProps<MenubarTriggerProps & { class?: HTMLAttributes["class"], value: string }>()
const delegatedProps = reactiveOmit(props, "class")

const forwardedProps = useForwardProps(delegatedProps)
</script>

<template>
  <MenubarTrigger
    data-slot="menubar-trigger"
    v-bind="forwardedProps"
    :class="cn(
      'focus:bg-panel-300 focus:text-text-100 text-text-100 data-[state=open]:bg-panel-300 data-[state=open]:text-text-100 flex items-center rounded-sm px-2 py-1 text-sm font-medium lg:text-[16px] outline-hidden select-none',
      props.class,
    )"
  >
    <slot/>
  </MenubarTrigger>
</template>

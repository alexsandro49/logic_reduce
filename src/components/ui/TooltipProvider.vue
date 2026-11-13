<script setup lang="ts">
/* eslint-disable vue/max-len */

import { TooltipProvider as TooltipProviderPrimitive, type TooltipProviderProps } from 'reka-ui';

defineProps<TooltipProviderProps>();
</script>

<template>
  <TooltipProviderPrimitive v-bind="$props">
    <slot />
  </TooltipProviderPrimitive>
</template>

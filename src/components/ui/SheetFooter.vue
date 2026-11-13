<script setup lang="ts">
/* eslint-disable vue/max-len */

import { cn } from '@/lib/utils';

defineProps<{
  class?: string
}>();
</script>

<template>
  <div :class="cn('flex flex-col-reverse sm:flex-row sm:justify-end sm:space-x-2', $props.class)">
    <slot />
  </div>
</template>

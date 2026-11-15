<script setup lang="ts">
/* eslint-disable vue/max-len */

import { cn } from '@/lib/utils';

defineProps<{
  class?: string
}>();
</script>

<template>
  <span :class="cn('ml-auto text-xs tracking-widest text-muted-foreground', $props.class)">
    <slot />
  </span>
</template>

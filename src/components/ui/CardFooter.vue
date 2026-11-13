<script setup lang="ts">
/* eslint-disable vue/max-len */

import { cn } from '@/lib/utils';

defineProps<{
  class?: string
}>();
</script>

<template>
  <div
    :class="cn('flex items-center p-6 border-t-3 border-foreground bg-muted/50', $props.class)"
  >
    <slot />
  </div>
</template>

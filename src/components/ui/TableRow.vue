<script setup lang="ts">
/* eslint-disable vue/max-len */

import { cn } from '@/lib/utils';

defineProps<{
  class?: string
}>();
</script>

<template>
  <tr
    :class="
      cn(
        'border-b-3 border-foreground transition-colors hover:bg-muted/50 data-[state=selected]:bg-accent',
        $props.class
      )
    "
  >
    <slot />
  </tr>
</template>

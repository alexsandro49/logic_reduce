<script setup lang="ts">
/* eslint-disable vue/max-len */

import { cn } from '@/lib/utils';

interface TimelineTimeProps {
  datetime?: string
  class?: string
}

const props = defineProps<TimelineTimeProps>();
</script>

<template>
  <time :datetime="datetime" :class="cn('text-xs font-medium text-muted-foreground', props.class)">
    <slot />
  </time>
</template>

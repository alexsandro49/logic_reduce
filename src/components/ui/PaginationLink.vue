<script setup lang="ts">
/* eslint-disable vue/require-default-prop */

import { cn } from '@/lib/utils';
import { buttonVariants } from './button-variants';
import { ClassValue } from 'clsx';

interface Props {
  class?: ClassValue
  isActive?: boolean
  size?: 'default' | 'sm' | 'lg' | 'icon'
  href?: string
}

const props = withDefaults(defineProps<Props>(), {
  size: 'icon',
});
</script>

<template>
  <a
    :href="href"
    :aria-current="isActive ? 'page' : undefined"
    :class="
      cn(
        buttonVariants({
          variant: isActive ? 'default' : 'outline',
          size: props.size,
        }),
        props.class
      )
    "
  >
    <slot />
  </a>
</template>

<script setup lang="ts">
import { PhArrowsClockwise, PhCheckFat, PhTrash } from '@phosphor-icons/vue';
import { useConfigStore } from '../stores/config';
import { useSimplificationStore } from '../stores/simplification';

const props = defineProps(['checkButtonLabel', 'checkButtonTooltip', 'trashButtonLabel',
  'trashButtonTooltip', 'refreshButtonLabel', 'refreshButtonTooltip'] as string[]);
const emit = defineEmits(['checkButton', 'trashButton', 'refreshButton']);

const configStore = useConfigStore();
const simplificationStore = useSimplificationStore();

function simplificationHelper() {
  emit('checkButton');
}

function trashButtonHelper() {
  emit('trashButton');
}

function refreshButtonHelper() {
  emit('refreshButton');
}
</script>

<template>
  <button
    @click="simplificationHelper"
    class="action-button"
    :style="{backgroundColor: configStore.getColor('action-button')}"
    type="button"
    :aria-label="props.checkButtonLabel"
    :disabled="simplificationStore.isSimplifying"
  >
    <PhCheckFat
      :size="32"
      :color="configStore.getColor('icon-color')"
      weight="fill"
    />
    <span
      class="tooltip"
      :style="{backgroundColor: configStore.getColor('base')}"
    >{{ props.checkButtonTooltip }}</span>
  </button>
  <button
    @click="trashButtonHelper"
    class="action-button"
    :style="{backgroundColor: configStore.getColor('action-button')}"
    type="button"
    :aria-label="props.trashButtonLabel"
  >
    <PhTrash
      :size="32"
      :color="configStore.getColor('icon-color')"
      weight="fill"
    />
    <span
      class="tooltip"
      :style="{backgroundColor: configStore.getColor('base')}"
    >{{ props.trashButtonTooltip }}</span>
  </button>
  <button
    class="action-button"
    :style="{backgroundColor: configStore.getColor('action-button')}"
    type="button"
    :aria-label="props.refreshButtonLabel"
    :disabled="simplificationStore.isSimplifying"
    @click="refreshButtonHelper"
  >
    <PhArrowsClockwise
      :size="32"
      :color="configStore.getColor('icon-color')"
      weight="bold"
    />
    <span
      class="tooltip"
      :style="{backgroundColor: configStore.getColor('base')}"
    >{{ props.refreshButtonTooltip }}</span>
  </button>
</template>

<style scoped>
@reference "../assets/main.css";

:global(html), :global(body), :global(#app) { @apply h-full overflow-hidden; }

.action-button { @apply relative; }
.action-button, .page-footer button { @apply flex h-[45px] w-[45px] items-center justify-center rounded-[10px] border-[1.5px] p-0 text-[27px] leading-none font-bold hover:-translate-y-px hover:brightness-[.96] focus-visible:outline-2 focus-visible:outline-offset-2 cursor-pointer; }

.tooltip, .footer-tooltip { @apply absolute hidden whitespace-nowrap rounded px-[6px] py-1 font-sans text-[12px] leading-normal font-black text-white; }
.tooltip { @apply -bottom-[31px] -left-3; }
.action-button:hover .tooltip,
.action-button:focus-visible .tooltip,
.action-button:hover .footer-tooltip,
.action-button:focus-visible .footer-tooltip { @apply block; }
</style>
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

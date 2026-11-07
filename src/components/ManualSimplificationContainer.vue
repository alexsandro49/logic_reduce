<script setup lang="ts">
import { PhCaretDown, PhX } from '@phosphor-icons/vue';
import { useConfigStore } from '../stores/config';
import { useSimplificationStore } from '../stores/simplification';
import { lawsSelect } from '../utils/shared';

const configStore = useConfigStore();
const simplificationStore = useSimplificationStore();

const props = defineProps<{
  verificationResults: boolean[];
}>();

function stepColorBackgroundHelper(index: number) {
  if (props.verificationResults[index] === undefined) {
    return 'transparent';
  } else if (props.verificationResults[index] === true) {
    return 'action-button';
  } else {
    return 'error';
  }
}

function selectColorBackgroundHelper(index: number) {
  if (props.verificationResults[index] === undefined) {
    return 'board';
  } else if (props.verificationResults[index] === true) {
    return 'action-button';
  } else {
    return 'error';
  }
}
</script>

<template>
  <div class="steps-card py-3" :style="{backgroundColor: configStore.getColor('board')}">
    <div class="steps-header" aria-hidden="true">
      <span class="step-number-heading"><span>#</span></span>
      <span>Passo</span>
      <span class="col-start-4 justify-self-center">Remover</span>
      <span>Identidade</span>
    </div>

    <article
      v-for="(step, index) in simplificationStore.steps" :key="index"
      class="step
      w-full h-12 mb-5 hover:bg-(--hightlight-hover) group" :style="{
        '--hightlight-hover': configStore.getColor('base'),
        '--law-background': configStore.getColor('board'),

        '--step-feedback-background':
          configStore.getColor(stepColorBackgroundHelper(index)),
        '--step-feedback-border': configStore.getColor(
          verificationResults[index] === undefined ? 'transparent' : 'background'),

        '--input-hover': configStore.getColor('board')
    }"
    >
      <div class="contents">
        <h2 :style="{ color: configStore.getColor('background') }">
          {{ index + 1 }}.
          <span v-if="verificationResults[index] !== undefined" class="sr-only">
            {{ verificationResults[index] ? 'Passo válido'
              : 'Passo com erro na expressão ou na lei selecionada' }}
          </span>
        </h2>
        <input
          v-model="step.value" class="form-input min-w-0 max-w-full justify-self-start
          group-hover:bg-(--input-hover)"
        >
      </div>

      <div class="contents">
        <button
          class="action-button col-start-4 group-hover:bg-(--law-background)
          invisible group-hover:visible hover:bg-(--remove-hover)"
          :style="{'--remove-hover': configStore.getColor('action-button')}" type="button"
          @click="simplificationStore.removeStep(index)" aria-label="Adicionar passo"
        >
          <PhX
            :size="32" :color="configStore.getColor('icon-color')"
            weight="bold"
          />
          <span
            class="tooltip" 
            :style="{ backgroundColor: configStore.getColor('base') }"
          >REMOVER</span>
        </button>
        <div class="law-control">
          <select
            v-model="step.law" class="form-input group-hover:bg-(--select-hover)"
            :style="{
              '--select-hover': configStore.getColor(selectColorBackgroundHelper(index))
            }"
            aria-label="Lei aplicada para obter o próximo passo"
          >
            <option
              v-for="notationOption in lawsSelect" :value="notationOption.value"
              :key="notationOption.value"
            >
              {{ notationOption.text }}
            </option>
          </select>
          <PhCaretDown
            class="law-caret" :size="16"
            aria-hidden="true"
          />
        </div>
      </div>
    </article>
  </div>
</template>

<style scoped>
@reference "../assets/main.css";

.law-control {
  @apply relative col-start-5;
}

@layer components {
  .law-control select {
    @apply appearance-none bg-none pr-9;
    background-color: var(--step-feedback-background, var(--law-background));
  }
}

.law-caret {
  @apply pointer-events-none absolute right-[13px] top-1/2 -translate-y-1/2 text-[#242424];
}

.steps-card {
  @apply grid content-start gap-x-3;
  grid-template-columns:
    max-content minmax(0, max-content) minmax(0, 1fr) 45px max-content;
  @apply max-[760px]:py-[23px];
}

.steps-header,
.step {
  @apply col-span-full grid grid-cols-subgrid items-center;
}

.steps-header {
  @apply mb-2 text-center text-sm font-black uppercase;
}

.step-number-heading {
  @apply grid justify-self-start;
}

.step-number-heading::before {
  content: '1.';
  @apply invisible col-start-1 row-start-1 h-0 font-normal;
}

.step-number-heading>span {
  @apply col-start-1 row-start-1;
}

.step-number-heading::before,
.step h2 {
  @apply rounded-[10px] border-[1.5px] px-2 text-4xl leading-[1.25];
  @apply max-[760px]:text-[22px];
}

.step h2 {
  @apply justify-self-start;
  background-color: var(--step-feedback-background, transparent);
  border-color: var(--step-feedback-border, transparent);
}
</style>

<script setup lang="ts">
import { PhCaretDown, PhX } from '@phosphor-icons/vue';
import { useConfigStore } from '../stores/config';
import { useSimplificationStore } from '../stores/simplification';

const configStore = useConfigStore();
const simplificationStore = useSimplificationStore();

const props = defineProps<{
    verificationResults: boolean[];
}>();

const rules = [
  { text: 'IDENTIDADE', value: 'identity' },
  { text: 'DOMINAÇÃO', value: 'null' },
  { text: 'IDEMPOTÊNCIA', value: 'idempotent' },
  { text: 'COMPLEMENTO', value: 'complement' },
  { text: 'D. NEGAÇÃO', value: 'double_negative' },
  { text: 'COMUTATIVA', value: 'commutative' },
  { text: 'ASSOCIATIVA', value: 'associative' },
  { text: 'DISTRIBUTIVA', value: 'distributive' },
  { text: 'ABSORÇÃO', value: 'absorption' },
  { text: 'D. MORGAN', value: 'de_morgan' },
  { text: 'FINAL', value: 'final' },
];

function stepColorBackgroundHelper(index: number) {
  if (props.verificationResults[index] === undefined) {
    return 'transparent';
  } else if (props.verificationResults[index] === true) {
    return 'action-button';
  } else {
    return 'error';
  }
}
</script>

<template>
  <div
    class="steps-card py-3"
    :style="{ backgroundColor: configStore.getColor('board') }"
  >
    <article
      v-for="(step, index) in simplificationStore.steps"
      :key="index"
      class="step flex w-full justify-between h-12 mb-5 hover:bg-(--cor-hover)
      hover:text-white group"
      :style="{
        '--cor-hover': configStore.getColor('base'),
        '--rule-background': configStore.getColor('board'),

        '--step-feedback-background':
          configStore.getColor(stepColorBackgroundHelper(index)),
        '--step-feedback-text': configStore.getColor('background'),
        '--step-feedback-border': configStore.getColor(
          verificationResults[index] === undefined ? 'transparent' : 'background'),
      }"
    >
      <div class="flex items-center gap-4">
        <h2
          class="text-(--step-feedback-text"
          :class="{'group-hover:text-(--step-feedback-text)':
            verificationResults[index] !== undefined}"
        >
          {{ index + 1 }}.
          <span
            v-if="verificationResults[index] !== undefined"
            class="sr-only"
          >
            {{ verificationResults[index] ? 'Passo válido'
              : 'Passo com erro na expressão ou na lei selecionada' }}
          </span>
        </h2>
        <input
          v-model="step.value"
          :style="{ '--input-hover': configStore.getColor('board') }"
          class="form-input group-hover:bg-(--input-hover)"
        >
      </div>

      <div class="flex items-center gap-3">
        <button
          class="action-button"
          :style="{ backgroundColor: configStore.getColor('action-button') }"
          type="button"
          @click="simplificationStore.removeStep(index)"
          aria-label="Adicionar passo"
        >
          <PhX
            :size="32"
            :color="configStore.getColor('icon-color')"
            weight="bold"
          />
          <span
            class="tooltip"
            :style="{ backgroundColor: configStore.getColor('base') }"
          >REMOVER</span>
        </button>
        <div class="rule-control">
          <select
            v-model="step.rule"
            class="form-input"
            aria-label="Lei aplicada para obter o próximo passo"
          >
            <option
              v-for="notationOption in rules"
              :value="notationOption.value"
              :key="notationOption.value"
            >
              {{ notationOption.text }}
            </option>
          </select>
          <PhCaretDown
            class="rule-caret"
            :size="16"
            aria-hidden="true"
          />
        </div>
      </div>
    </article>
  </div>
</template>

<style scoped>
@reference "../assets/main.css";

.rule-control {
  @apply relative;
}

.rule-control select {
  @apply appearance-none bg-none pr-9;
  background-color: var(--step-feedback-background, var(--rule-background));
}

.rule-caret {
  @apply pointer-events-none absolute right-[13px] top-1/2 -translate-y-1/2 text-[#242424];
}

.steps-card {
  @apply max-[760px]:py-[23px];
}

.step h2 {
  @apply rounded-[10px] border-[1.5px] px-2 text-4xl leading-[1.25];
  @apply max-[760px]:text-[22px];
  background-color: var(--step-feedback-background, transparent);
  border-color: var(--step-feedback-border, transparent);
}
</style>

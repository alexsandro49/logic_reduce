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
          class="group-hover:bg-(--input-hover)"
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

:global(html),
:global(body),
:global(#app) {
    @apply h-full overflow-hidden;
}

.page-shell {
    @apply relative flex h-dvh max-h-dvh min-h-0 flex-col overflow-hidden box-border rounded-b-[20px] bg-white font-roboto text-[#242424] max-[760px]:px-3.5;
}

.page-shell :deep(.app-header) {
    @apply shrink-0;
}

.workspace {
    @apply flex min-h-0 flex-1 flex-col pt-[43px] pb-[97px] max-[760px]:pt-7;
}

.controls-row {
    @apply flex shrink-0 items-end gap-3.5 max-[760px]:flex-wrap max-[760px]:items-stretch;
}

.field {
    @apply flex flex-col gap-[7px];
}

.field>span,
.steps-section h1,
.result-row h2 {
    @apply text-[19px] tracking-[.1px] uppercase;
}

input,
select,
.result-value {
    @apply box-border h-[45px] rounded-[10px] border-[1.5px] border-[#242424] px-[13px] text-[18px] leading-[1.2] text-[#242424] outline-none focus:outline-2 focus:outline-offset-2;
}

.expression-field {
    @apply w-[367px] max-[760px]:w-full;
}

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

.notation-control {
    @apply flex items-end gap-[13px] max-[760px]:w-full max-[760px]:flex-wrap max-[760px]:items-stretch;
}

.notation-field {
    @apply relative max-[760px]:flex-1 after:pointer-events-none after:absolute after:right-[14px] after:bottom-[18px] after:border-x-[5px] after:border-t-[7px] after:border-x-transparent after:border-t-[#242424];
}

.notation-field select {
    @apply w-41 appearance-none pr-10 text-[#242424] disabled:opacity-100 max-[760px]:w-full;
}

.notation-field select option {
    @apply text-[#242424];
}

.action-button {
    @apply relative;
}

.action-button,
.page-footer button {
    @apply flex h-[45px] w-[45px] items-center justify-center rounded-[10px] border-[1.5px] p-0 text-[27px] leading-none font-bold hover:-translate-y-px hover:brightness-[.96] focus-visible:outline-2 focus-visible:outline-offset-2 cursor-pointer;
}

.action-button-img {
    @apply w-7 h-7
}

.ufal-button-img {
    @apply w-10 h-10 cursor-pointer
}

.font-text {
    @apply font-black
}

.steps-section {
    @apply mt-7 flex min-h-0 flex-1 flex-col;
}

.steps-card {
    @apply min-h-0 flex-1 overflow-x-hidden overflow-y-auto box-border rounded-[23px] border-[1.5px] border-[#242424] px-8 max-[760px]:px-[18px] max-[760px]:py-[23px];
}

.step h2 {
    @apply rounded-[10px] border-[1.5px] px-2 text-4xl leading-[1.25] max-[760px]:text-[22px];
    background-color: var(--step-feedback-background, transparent);
    border-color: var(--step-feedback-border, transparent);
}

.step-expression,
.step-description {
    @apply mb-4 text-3xl leading-[1.45] max-[760px]:text-[14px] max-[760px]:[overflow-wrap:anywhere];
}

.step-expression {
    @apply font-inter;
}

.step-description {
    @apply ml-3;
}

.result-row {
    @apply mt-7 flex shrink-0 items-center gap-[7px] max-[760px]:flex-wrap max-[760px]:items-start;
}

.result-row h2 {
    @apply whitespace-nowrap;
}

.result-value {
    @apply flex h-[31px] min-w-[192px] items-center text-[13px];
}

.copy-button {
    @apply relative flex h-7 w-[25px] items-center justify-center border-0 bg-transparent p-0 text-[22px] hover:-translate-y-px focus-visible:outline-2 focus-visible:outline-offset-2 cursor-pointer;
}

.tooltip,
.footer-tooltip {
    @apply absolute hidden whitespace-nowrap rounded px-[6px] py-1 font-sans text-[12px] leading-normal font-black text-white;
}

.tooltip {
    @apply -bottom-[31px] -left-3;
}

.footer-tooltip {
    @apply bottom-full right-0 mb-[5px];
}

.copy-button:hover .tooltip,
.copy-button:focus-visible .tooltip,
.action-button:hover .tooltip,
.action-button:focus-visible .tooltip,
.action-button:hover .footer-tooltip,
.action-button:focus-visible .footer-tooltip {
    @apply block;
}

.page-footer {
    @apply absolute -right-px -bottom-px flex min-h-[72px] items-center gap-7 rounded-tl-[19px] rounded-br-[19px] border-[1.5px] border-r-0 border-b-0 py-0 pr-[27px] pl-4 max-[760px]:gap-2.5 max-[760px]:px-3;
}

.page-footer strong {
    @apply text-[36px] leading-none font-black;
}

.page-footer button {
    @apply w-[59px];
}

input::placeholder {
    color: var(--placeholder-color);
}
</style>

<script setup lang="ts">
import HeaderComponent from '../components/HeaderComponent.vue';

import ufalIcon from '@/assets/ufal-logo.svg';
import { ref, watch } from 'vue';
import { useConfigStore } from '../stores/config.ts';
import { PhBook, PhCopySimple, PhEye, PhGithubLogo, PhLightbulbFilament, PhPlus }
  from '@phosphor-icons/vue';
import { useSimplificationStore } from '../stores/simplification.ts';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import ActionButtons from '../components/ActionButtons.vue';
import AutomaticSimplificationContainer
  from '../components/AutomaticSimplificationContainer.vue';
import ManualSimplificationContainer
  from '../components/ManualSimplificationContainer.vue';
import { invoke } from '@tauri-apps/api/core';
import type { EquivalenceResult } from '../utils/types.ts';

const configStore = useConfigStore();
const simplificationStore = useSimplificationStore();

const copyButtonTittle = ref('COPIAR');
const automaticSimplification = ref(false);

const isVerifying = ref(false);
const verificationMessage = ref('');
const verificationError = ref('');
let verificationVersion = 0;

watch(() => [
  simplificationStore.steps.map(step => step.value),
  automaticSimplification.value,
  simplificationStore.submittedExpression,
  simplificationStore.result,
], () => {
  verificationVersion++;
  isVerifying.value = false;
  verificationMessage.value = '';
  verificationError.value = '';
}, { deep: true, flush: 'sync' });

function footerColorHelper(): string {
  const value = configStore.darkTheme ? 'board' : 'icon-color';
  return configStore.getColor(value);
}

async function copyResult() {
  await writeText(simplificationStore.result);
  copyButtonTittle.value = 'COPIADO!';
  setTimeout(() => {
    copyButtonTittle.value = 'COPIAR';
  }, 1000);
}

function automaticSimplificationHelper() {
  simplificationStore.simplification();
  automaticSimplification.value = !automaticSimplification.value;
}

async function verifyTruthTable() {
  if (isVerifying.value) return;
  verificationMessage.value = '';
  verificationError.value = '';
  if (!automaticSimplification.value && simplificationStore.steps.length < 2) {
    verificationError.value = 'Adicione a expressão original e uma tentativa de simplificação.';
    return;
  }
  const original = automaticSimplification.value
    ? simplificationStore.submittedExpression
    : simplificationStore.steps[0].value;
  const candidate = automaticSimplification.value
    ? simplificationStore.result
    : simplificationStore.steps[simplificationStore.steps.length - 1].value;
  if (!original.trim() || !candidate.trim()) {
    verificationError.value = 'Informe as duas expressões antes de verificar.';
    return;
  }

  const version = ++verificationVersion;
  isVerifying.value = true;
  try {
    const result = await invoke<EquivalenceResult>('verify_equivalence', { original, candidate });
    if (version !== verificationVersion) return;
    if (result.equivalent) {
      verificationMessage.value = 'As expressões são equivalentes.';
    } else if (result.counterexample) {
      const example = result.counterexample;
      const inputs = Object.entries(example.inputs)
        .map(([name, value]) => `${name} = ${Number(value)}`).join(', ');
      verificationMessage.value = `As expressões não são equivalentes. ${
        inputs ? `Para ${inputs}: ` : ''
      }original = ${Number(example.original)}, tentativa = ${Number(example.candidate)}.`;
    }
  } catch (error) {
    if (version !== verificationVersion) return;
    verificationError.value = typeof error === 'string' ? error
      : error instanceof Error ? error.message : 'Não foi possível verificar as expressões.';
  } finally {
    if (version === verificationVersion) isVerifying.value = false;
  }
}
</script>

<template>
  <main class="page-shell" :class="{ dark: configStore.darkTheme }">
    <HeaderComponent :active-window-button="0" />
    <section class="workspace" :style="{
      backgroundColor: configStore.darkTheme ?
        configStore.getColor('background') : '#FFFFFF'
    }" aria-label="Simplificação de expressão booleana">
      <div class="controls-row">
        <label class="field expression-field">
          <span class="font-text" :style="{ color: configStore.getColor('text') }">Expressão booleana:</span>
          <input v-model="simplificationStore.expression" placeholder="~((~A + B) & (~B + C))" :style="{
            backgroundColor: configStore.getColor('input'),
            color: configStore.getColor('input-text'),
            '--placeholder-color': configStore.getColor('input-text')
          }" type="text" aria-label="Expressão booleana">
        </label>
        <div class="notation-control w-full">
          <button class="action-button self-stretch" :style="{ backgroundColor: configStore.getColor('action-button') }"
            type="button" @click="simplificationStore.addStep" aria-label="Adicionar passo"
            v-if="!automaticSimplification">
            <PhPlus :size="32" :color="configStore.getColor('icon-color')" weight="bold" />
            <span class="tooltip" :style="{ backgroundColor: configStore.getColor('base') }">ADICIONAR</span>
          </button>

          <ActionButtons @check-button="simplificationStore.simplification"
            @trash-button="simplificationStore.clearData" @refresh-button="simplificationStore.randomExpressionHelper"
            check-button-label="Simplificar expressão" trash-button-label="Limpar expressão"
            refresh-button-label="Expressão aleatória" check-button-tooltip="SIMPLIFICAR" trash-button-tooltip="LIMPAR"
            refresh-button-tooltip="ALEATÓRIO" />

          <div class="notation-control ml-auto">
            <button class="action-button" :style="{ backgroundColor: configStore.getColor('action-button') }"
              type="button" @click="verifyTruthTable" aria-label="Verificar equivalência das expressões"
              :disabled="isVerifying || simplificationStore.isSimplifying" :aria-busy="isVerifying">
              <PhLightbulbFilament :size="32" :color="configStore.getColor('icon-color')" weight="fill" />
              <span class="tooltip" :style="{ backgroundColor: configStore.getColor('base') }">{{
                isVerifying ? 'VERIFICANDO...' : 'VERIFICAR EQUIVALÊNCIA'
              }}</span>
            </button>
            <button class="action-button" :style="{
              backgroundColor: automaticSimplification ?
                configStore.getColor('board') : configStore.getColor('action-button')
            }" type="button" @click="automaticSimplificationHelper" aria-label="Adicionar passo">
              <PhEye :size="32" :color="configStore.getColor('icon-color')" weight="fill" />
              <span class="tooltip tooltip-end" :style="{ backgroundColor: configStore.getColor('base') }">SIMPLIFICAÇÃO
                AUTOMÁTICA</span>
            </button>
          </div>
        </div>
      </div>
      <section class="steps-section" aria-labelledby="steps-title">
        <h1 id="steps-title" class="font-text" :style="{ color: configStore.getColor('text') }">
          Passos:
        </h1>

        <AutomaticSimplificationContainer v-if="automaticSimplification" />
        <ManualSimplificationContainer v-else />

        <p v-if="simplificationStore.error" role="alert">
          {{ simplificationStore.error }}
        </p>
        <p v-if="verificationMessage" :style="{ color: configStore.darkTheme ? 
          configStore.getColor('board') : configStore.getColor('background')}"
          role="status">{{ verificationMessage }}</p>
        <p v-if="verificationError" :style="{ color: configStore.darkTheme ? 
          configStore.getColor('board') : configStore.getColor('background')}"
          role="alert">{{ verificationError }}</p>
      </section>
      <section class="result-row" aria-label="Resultado simplificado">
        <h2 class="font-text" :style="{ color: configStore.getColor('text') }">
          Forma simplificada:
        </h2>
        <output class="result-value" :style="{ backgroundColor: configStore.getColor('board') }">{{
          simplificationStore.result }}</output>
        <button class="copy-button" type="button" aria-label="Alternar tema" @click="copyResult"
          :disabled="simplificationStore.result === ''">
          <PhCopySimple :size="32" :color="configStore.getColor('text')" weight="fill" />
          <span class="tooltip" :style="{ backgroundColor: configStore.getColor('base') }">{{ copyButtonTittle }}</span>
        </button>
      </section>
    </section>
    <footer class="page-footer" :style="{ borderColor: footerColorHelper() }">
      <strong :style="{ color: footerColorHelper() }">2026</strong>
      <button class="action-button" :style="{ backgroundColor: configStore.getColor('footer-buttons-background') }"
        type="button" aria-label="Alternar tema">
        <img :src="ufalIcon" class="ufal-button-img" alt="Moon icon" aria-hidden="true">
        <span class="footer-tooltip" :style="{ backgroundColor: configStore.getColor('base') }">UFAL</span>
      </button>
      <button class="action-button" :style="{ backgroundColor: configStore.getColor('footer-buttons-background') }"
        type="button" aria-label="Alternar tema">
        <PhBook :size="32" :color="configStore.getColor('footer-buttons-color')" weight="fill" />
        <span class="footer-tooltip" :style="{ backgroundColor: configStore.getColor('base') }">TRABALHO
          RELACIONADO</span>
      </button>
      <button class="action-button" :style="{ backgroundColor: configStore.getColor('footer-buttons-background') }"
        type="button" aria-label="Alternar tema">
        <PhGithubLogo :size="32" :color="configStore.getColor('footer-buttons-color')" weight="fill" />
        <span class="footer-tooltip" :style="{ backgroundColor: configStore.getColor('base') }">REPOSITÓRIO</span>
      </button>
    </footer>
  </main>
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
  @apply flex px-8 min-h-0 flex-1 flex-col pt-[43px] pb-[97px] max-[760px]:pt-7;
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

.step h2 {
  @apply mb-[6px] text-4xl leading-[1.25] max-[760px]:text-[22px];
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

.tooltip-end {
  @apply left-auto right-0;
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

.hide-scrollbar {
  scrollbar-width: none;
  /* Firefox */
  -ms-overflow-style: none;
  /* IE 10+ */
}

.hide-scrollbar::-webkit-scrollbar {
  display: none;
  /* Chrome, Safari, Opera */
}
</style>

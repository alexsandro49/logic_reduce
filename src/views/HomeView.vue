<script setup lang="ts">
import HeaderComponent from '../components/HeaderComponent.vue';

import ufalIcon from '@/assets/ufal-logo.svg';
import { computed, ref, watch } from 'vue';
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
import type {
  EquivalenceResult, ManualCompletion, ManualStepValidation,
} from '../utils/types.ts';

const configStore = useConfigStore();
const simplificationStore = useSimplificationStore();

const copyButtonTittle = ref('COPIAR');
const automaticSimplification = ref(false);

const isVerifying = ref(false);
const verificationMessage = ref('');
const verificationError = ref('');
const stepVerificationResults = ref<boolean[]>([]);
const hintMessage = ref('');
const verificationFeedback = computed(() => hintMessage.value
  || verificationError.value || verificationMessage.value);
let verificationVersion = 0;

watch(() => [
  simplificationStore.steps.map(step => [step.value, step.rule]),
  automaticSimplification.value,
  simplificationStore.submittedExpression,
  simplificationStore.result,
], () => {
  verificationVersion++;
  isVerifying.value = false;
  verificationMessage.value = '';
  verificationError.value = '';
  stepVerificationResults.value = [];
  hintMessage.value = '';
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

async function checkExpression() {
  if (isVerifying.value || simplificationStore.isSimplifying) return;
  if (automaticSimplification.value) {
    await simplificationStore.simplification();
    if (simplificationStore.error) {
      verificationError.value = 'Não foi possível simplificar a expressão.';
      return;
    }
  }
  await verifyTruthTable();
}

async function verifyTruthTable(requestHint = false) {
  if (isVerifying.value) return;
  verificationMessage.value = '';
  verificationError.value = '';
  hintMessage.value = '';
  if (!automaticSimplification.value && simplificationStore.steps.length < 2) {
    verificationError.value = 'Passos insuficientes.';
    if (requestHint) {
      hintMessage.value =
        'Adicione a expressão original e uma tentativa de simplificação.';
    }
    return;
  }
  const original = automaticSimplification.value
    ? simplificationStore.submittedExpression
    : simplificationStore.steps[0].value;
  const lastStep = simplificationStore.steps[simplificationStore.steps.length - 1];
  const candidate = automaticSimplification.value
    ? simplificationStore.result
    : lastStep.value;
  if (!original.trim() || !candidate.trim()) {
    verificationError.value = 'Há expressões vazias.';
    if (requestHint) hintMessage.value = 'Informe as duas expressões antes de verificar.';
    return;
  }

  const version = ++verificationVersion;
  isVerifying.value = true;
  try {
    let results: EquivalenceResult[];
    let validations: ManualStepValidation[] = [];
    if (automaticSimplification.value) {
      results = [await invoke<EquivalenceResult>('verify_equivalence', {
        original, candidate,
      })];
    } else {
      validations = await Promise.all(
        simplificationStore.steps.slice(0, -1).map((step, index) =>
          invoke<ManualStepValidation>('verify_manual_step', {
            original,
            previous: step.value,
            candidate: simplificationStore.steps[index + 1].value,
            selectedRule: step.rule,
          }),
        ),
      );
      results = validations.map(validation => validation.equivalence);
    }
    if (version !== verificationVersion) return;
    if (!automaticSimplification.value) {
      stepVerificationResults.value = simplificationStore.steps.map((step, index) =>
        (index === 0 || results[index - 1].equivalent)
        && (index === validations.length
          ? step.rule === 'final' : validations[index].law_valid),
      );
    }
    const firstIncorrectIndex = results.findIndex((result, index) =>
      !result.equivalent || (!automaticSimplification.value
        && !validations[index].law_valid),
    );
    const resultIndex = firstIncorrectIndex === -1
      ? results.length - 1 : firstIncorrectIndex;
    const result = results[resultIndex];
    if (!result.equivalent) {
      verificationError.value = 'As expressões não são equivalentes.';
    }
    if (!automaticSimplification.value && firstIncorrectIndex !== -1) {
      const stepLabel = `Passo ${firstIncorrectIndex + 1}: `;
      verificationError.value = stepLabel + (result.equivalent
        ? 'lei incorreta.' : 'expressão incorreta.');
      if (requestHint) {
        hintMessage.value = stepLabel + validations[firstIncorrectIndex].hint;
      }
      return;
    }
    if (!automaticSimplification.value && lastStep.rule !== 'final') {
      verificationError.value = `Passo ${simplificationStore.steps.length}: `
        + 'conclusão pendente.';
      if (requestHint) {
        hintMessage.value = `Passo ${simplificationStore.steps.length}: `
          + 'Selecione a lei FINAL para concluir a simplificação.';
      }
      return;
    }
    if (result.equivalent) {
      if (!automaticSimplification.value) {
        const completion = await invoke<ManualCompletion>('manual_completion', {
          source: candidate,
        });
        if (version !== verificationVersion) return;
        if (completion.complete) {
          verificationMessage.value = 'Simplificação concluída.';
          if (requestHint) {
            hintMessage.value = 'Simplificação concluída. '
              + 'As expressões e as leis de todos os passos estão corretas.';
          }
        } else {
          verificationError.value = 'Simplificação incompleta.';
          if (requestHint) {
            hintMessage.value = completion.next_rule
              ? `Tentativa incompleta. Continue com a lei ${completion.next_rule}.`
              : 'Os passos estão corretos, '
                + 'mas a última expressão ainda pode ser simplificada.';
          }
        }
      } else {
        verificationMessage.value = 'As expressões são equivalentes.';
      }
    }
  } catch (error) {
    if (version !== verificationVersion) return;
    verificationError.value = 'Não foi possível verificar as expressões.';
    if (requestHint) {
      hintMessage.value = typeof error === 'string' ? error : error instanceof Error
        ? error.message : verificationError.value;
    }
  } finally {
    if (version === verificationVersion) isVerifying.value = false;
  }
}
</script>

<template>
  <main
    class="page-shell"
    :class="{ dark: configStore.darkTheme }"
    :style="{
      backgroundColor: configStore.darkTheme ?
        configStore.getColor('background') : '#FFFFFF'
    }"
  >
    <HeaderComponent :active-window-button="0" />
    <section
      class="workspace"
      aria-label="Simplificação de expressão booleana"
    >
      <div class="controls-row">
        <label class="field expression-field">
          <span
            class="font-text"
            :style="{ color: configStore.getColor('text') }"
          >Expressão booleana:</span>
          <input
            v-model="simplificationStore.expression"
            placeholder="~((~A + B) & (~B + C))"
            :style="{
              backgroundColor: configStore.getColor('input'),
              color: configStore.getColor('input-text'),
              '--placeholder-color': configStore.getColor('input-text')
            }"
            type="text"
            aria-label="Expressão booleana"
          >
        </label>
        <div class="action-controls">
          <button
            class="action-button"
            :style="{ backgroundColor: configStore.getColor('action-button') }"
            type="button"
            @click="simplificationStore.addStep"
            aria-label="Adicionar passo"
            v-if="!automaticSimplification"
          >
            <PhPlus
              :size="32"
              :color="configStore.getColor('icon-color')"
              weight="bold"
            />
            <span
              class="tooltip"
              :style="{ backgroundColor: configStore.getColor('base') }"
            >ADICIONAR</span>
          </button>

          <ActionButtons
            @check-button="checkExpression"
            @trash-button="simplificationStore.clearData"
            @refresh-button="simplificationStore.randomExpressionHelper"
            check-button-label="Simplificar expressão"
            trash-button-label="Limpar expressão"
            refresh-button-label="Expressão aleatória"
            check-button-tooltip="SIMPLIFICAR"
            trash-button-tooltip="LIMPAR"
            refresh-button-tooltip="ALEATÓRIO"
          />
        </div>

        <div class="verification-controls">
          <p
            v-if="verificationFeedback"
            class="verification-feedback font-text"
            :style="{ color: configStore.getColor('text') }"
            :role="verificationError ? 'alert' : 'status'"
            aria-atomic="true"
            tabindex="0"
          >
            {{ verificationFeedback }}
          </p>
          <button
            v-if="!automaticSimplification"
            class="action-button"
            :style="{ backgroundColor: configStore.getColor('action-button') }"
            type="button"
            aria-label="Mostrar dica para a simplificação manual"
            aria-describedby="manual-hint-tooltip"
            :disabled="isVerifying || simplificationStore.isSimplifying"
            :aria-busy="isVerifying"
            @click="verifyTruthTable(true)"
          >
            <PhLightbulbFilament
              :size="32"
              :color="configStore.getColor('icon-color')"
              weight="fill"
            />
            <span
              id="manual-hint-tooltip"
              role="tooltip"
              class="tooltip"
              :style="{ backgroundColor: configStore.getColor('base') }"
            >DICA</span>
          </button>

          <button
            class="action-button"
            :style="{
              backgroundColor: automaticSimplification ?
                configStore.getColor('board') : configStore.getColor('action-button')
            }"
            type="button"
            @click="automaticSimplificationHelper"
            aria-label="Adicionar passo"
          >
            <PhEye
              :size="32"
              :color="configStore.getColor('icon-color')"
              weight="fill"
            />
            <span
              class="tooltip tooltip-end"
              :style="{ backgroundColor: configStore.getColor('base') }"
            >SIMPLIFICAÇÃO
              AUTOMÁTICA</span>
          </button>
        </div>
      </div>
      <section
        class="steps-section"
        aria-labelledby="steps-title"
      >
        <h1
          id="steps-title"
          class="font-text"
          :style="{ color: configStore.getColor('text') }"
        >
          Passos:
        </h1>

        <AutomaticSimplificationContainer v-if="automaticSimplification" />
        <ManualSimplificationContainer
          v-else
          :verification-results="stepVerificationResults"
        />

        <p
          v-if="simplificationStore.error"
          role="alert"
        >
          {{ simplificationStore.error }}
        </p>
      </section>
    </section>
    <div class="bottom-row">
      <section
        class="result-row"
        aria-label="Resultado simplificado"
        v-if="automaticSimplification"
      >
        <h2
          class="font-text"
          :style="{ color: configStore.getColor('text') }"
        >
          Forma simplificada:
        </h2>
        <output
          class="result-value"
          :style="{ backgroundColor: configStore.getColor('board') }"
        >{{
          simplificationStore.result }}</output>
        <button
          class="copy-button"
          type="button"
          aria-label="Alternar tema"
          @click="copyResult"
          :disabled="simplificationStore.result === ''"
        >
          <PhCopySimple
            :size="32"
            :color="configStore.getColor('text')"
            weight="fill"
          />
          <span
            class="tooltip"
            :style="{ backgroundColor: configStore.getColor('base') }"
          >{{ copyButtonTittle }}</span>
        </button>
      </section>
      <footer
        class="page-footer"
        :style="{ borderColor: footerColorHelper() }"
      >
        <strong :style="{ color: footerColorHelper() }">2026</strong>
        <button
          class="action-button"
          :style="{ backgroundColor: configStore.getColor('footer-buttons-background') }"
          type="button"
          aria-label="Alternar tema"
        >
          <img
            :src="ufalIcon"
            class="ufal-button-img"
            alt="Moon icon"
            aria-hidden="true"
          >
          <span
            class="footer-tooltip"
            :style="{ backgroundColor: configStore.getColor('base') }"
          >UFAL</span>
        </button>
        <button
          class="action-button"
          :style="{ backgroundColor: configStore.getColor('footer-buttons-background') }"
          type="button"
          aria-label="Alternar tema"
        >
          <PhBook
            :size="32"
            :color="configStore.getColor('footer-buttons-color')"
            weight="fill"
          />
          <span
            class="footer-tooltip"
            :style="{ backgroundColor: configStore.getColor('base') }"
          >TRABALHO
            RELACIONADO</span>
        </button>
        <button
          class="action-button"
          :style="{ backgroundColor: configStore.getColor('footer-buttons-background') }"
          type="button"
          aria-label="Alternar tema"
        >
          <PhGithubLogo
            :size="32"
            :color="configStore.getColor('footer-buttons-color')"
            weight="fill"
          />
          <span
            class="footer-tooltip"
            :style="{ backgroundColor: configStore.getColor('base') }"
          >REPOSITÓRIO</span>
        </button>
      </footer>
    </div>
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
  @apply grid min-h-0 flex-1 grid-rows-[auto_minmax(0,1fr)] gap-7 px-8;
  @apply pt-[43px] pb-7 max-[760px]:pt-7;
}

.controls-row {
  @apply grid h-[120px] shrink-0 grid-cols-[367px_auto_minmax(0,1fr)];
  @apply grid-rows-[28px_minmax(0,1fr)];
  @apply items-center gap-x-3.5 gap-y-[7px];
  @apply max-[1100px]:h-[168px] max-[1100px]:grid-cols-[minmax(0,1fr)_auto];
  @apply max-[1100px]:grid-rows-[28px_45px_minmax(0,1fr)];
  @apply max-[760px]:h-[264px] max-[760px]:grid-cols-1;
  @apply max-[760px]:grid-rows-[28px_45px_45px_minmax(0,1fr)];
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
  @apply row-span-2 grid min-w-0 grid-rows-subgrid items-center self-stretch;
}

.expression-field input {
  @apply min-w-0 w-full;
}

.action-controls {
  @apply col-start-2 row-start-2 flex items-center gap-[13px];
  @apply max-[760px]:col-start-1 max-[760px]:row-start-3;
}

.action-controls :deep(.action-button) {
  @apply shrink-0;
}

.verification-controls {
  @apply col-start-3 row-start-2 flex h-full min-h-0 min-w-0;
  @apply items-center justify-end gap-[13px];
  @apply max-[1100px]:col-span-full max-[1100px]:col-start-1;
  @apply max-[1100px]:row-start-3 max-[760px]:row-start-4 max-[760px]:flex-wrap;
  @apply max-[760px]:content-center;
}

.verification-feedback {
  @apply min-h-0 max-h-full min-w-0 max-w-lg flex-1 overflow-y-auto;
  @apply text-right text-[19px] leading-snug wrap-anywhere;
  @apply max-[760px]:basis-full max-[760px]:max-w-none max-[760px]:text-left;
  @apply max-[760px]:max-h-[calc(100%_-_58px)];
}

.verification-controls .action-button {
  @apply shrink-0;
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
  @apply grid min-h-0 grid-rows-[auto_minmax(0,1fr)_auto] overflow-hidden;
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

.bottom-row {
  @apply grid min-h-[72px] shrink-0 grid-cols-[minmax(0,1fr)_auto] items-start gap-7;
  @apply max-[1100px]:grid-cols-1 max-[1100px]:gap-0;
}

.result-row {
  @apply flex min-w-0 flex-wrap items-center gap-[7px] pl-8;
  @apply max-[1100px]:min-h-[72px] max-[1100px]:px-8;
}

.result-row h2 {
  @apply whitespace-nowrap;
}

.result-value {
  @apply flex h-[31px] min-w-[192px] max-w-full items-center overflow-x-auto text-[13px];
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
  @apply col-start-2 flex min-h-[72px] items-center justify-self-end gap-7;
  @apply rounded-tl-[19px] rounded-br-[19px] border-[1.5px] border-r-0 border-b-0;
  @apply py-0 pr-[27px] pl-4 max-[1100px]:col-start-1;
  @apply max-[760px]:gap-2.5 max-[760px]:px-3;
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

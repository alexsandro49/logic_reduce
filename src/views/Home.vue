<script setup lang="ts">
import HeaderComponent from '../components/Header.vue'
import { invoke } from '@tauri-apps/api/core'

type ReductionStep = { title: string; expression: string; description: string }

import checkIcon from '@/assets/check-fat.svg'
import deleteIcon from '@/assets/trash.svg'
import refreshIcon from '@/assets/arrows-clockwise.svg'
import copyBlackIcon from '@/assets/copy-black.svg'
import copyCyanIcon from '@/assets/copy-cyan.svg'
import ufalIcon from '@/assets/ufal-logo.svg'
import bookIcon from '@/assets/book.svg'
import githubIcon from '@/assets/github-logo.svg'
import { Ref, ref } from 'vue';
import { useConfigStore } from '../stores/config.ts';
import { generateRandomExpression } from '../utils/expressionGenerator.ts';
import { stepStruct } from '../utils/types.ts';

const configStore = useConfigStore();

const expression = ref('')
const result = ref('');
const submittedExpression = ref('');
const error = ref('');
const isSimplifying = ref(false);

const notations = [
  { text:"PADRÃO", value: "default"},
  { text:"LÓGICA", value: "logic"},
  { text:"MATEMÁTICA", value: "mathematical"},
  { text:"PROGBOOLS", value: "progbools"},
  { text:"PROGBITS", value: "progbits"},
  { text:"ALTLOGIC", value: "altlogic"},
  { text:"LATEX", value: "latex"}
]
const notation = ref("default");
const steps: Ref<ReductionStep[]> = ref([]);

async function simplification() {
  const source = expression.value.trim() || '~((~A + B) & (~B + C))';

  isSimplifying.value = true;
  error.value = '';
  submittedExpression.value = source;
  steps.value = [];
  result.value = '';

  try {
    const [simplifiedExpression, newSteps] = await invoke<[string, stepStruct[]]>(
      'simplify_helper',
      { source, notation: notation.value },
    );

    result.value = simplifiedExpression;
    steps.value = newSteps.map((step) => ({
      title: step.rule_name,
      expression: step.after_rule,
      description: 'Regra aplicada durante a simplificação.',
    }));
  } catch (reason) {
    error.value = typeof reason === 'string' ? reason : 'Não foi possível simplificar a expressão.';
  } finally {
    isSimplifying.value = false;
  }
}

function clearData() {
  isSimplifying.value = false;
  error.value = '';
  steps.value = [];
  result.value = '';
  expression.value = ''
}

function randomExpressionHelper() {
  clearData()
  expression.value = generateRandomExpression(expression)
}
</script>

<template>
  <main class="page-shell" :class="{dark: configStore.darkTheme }">
    <HeaderComponent :active-window-button="0"/>
    <section class="workspace px-7" aria-label="Simplificação de expressão booleana">
      <div class="controls-row">
        <label class="field expression-field">
          <span class="font-text">Expressão booleana:</span>
          <input v-model="expression" placeholder="~((~A + B) & (~B + C))" type="text" aria-label="Expressão booleana" />
        </label>
        <div class="notation-control">
          <label class="field notation-field">
            <span class="font-text">Notação:</span>
            <select v-model="notation" aria-label="Notação">
              <option v-for="notation in notations" :value="notation.value" :key="notation.value">
                {{ notation.text }}
              </option>
            </select>
          </label>
          <button class="action-button" type="button" aria-label="Simplificar expressão" :disabled="isSimplifying" @click="simplification">
            <img :src="checkIcon" class="action-button-img" alt="" aria-hidden="true" />
          </button>
          <button @click="clearData" class="action-button" type="button" aria-label="Alternar tema">
            <img :src="deleteIcon" class="action-button-img" alt="Moon icon" aria-hidden="true" />
          </button>
          <button class="action-button" type="button" aria-label="Expressão aleatória" :disabled="isSimplifying" @click="randomExpressionHelper">
            <img :src="refreshIcon" class="action-button-img" alt="Moon icon" aria-hidden="true" />
          </button>
        </div>
      </div>
      <section class="steps-section" aria-labelledby="steps-title">
        <h1 id="steps-title" class="font-text">Passos:</h1>
        <div class="steps-card">
          <article v-for="(step, index) in steps" :key="step.title" class="step">
            <h2>{{ index + 1 }}. <span>{{ step.title }}</span></h2>
            
            <p v-if="index == 0" class="step-expression">{{ submittedExpression }} -> {{ step.expression }}</p>
            <p v-else class="step-expression">{{ steps[index - 1].expression }} -> {{ step.expression }}</p>
            
            <p class="step-description">{{ step.description }}</p>
          </article>
        </div>
        <p v-if="error" role="alert">{{ error }}</p>
      </section>
      <section class="result-row" aria-label="Resultado simplificado">
        <h2 class="font-text">Forma simplificada:</h2>
        <output class="result-value">{{ result }}</output>
        <button class="copy-button" type="button" aria-label="Alternar tema">
          <img v-if="configStore.darkTheme" :src="copyCyanIcon" alt="Moon icon" aria-hidden="true" />
          <img v-else :src="copyBlackIcon" alt="Moon icon" aria-hidden="true" />
        </button>
      </section>
    </section>
    <footer class="page-footer">
      <strong>{{new Date().getFullYear()}}</strong>
      <button class="action-button" type="button" aria-label="Alternar tema">
        <img :src="ufalIcon" class="ufal-button-img" alt="Moon icon" aria-hidden="true" />
      </button>
      <button class="action-button" type="button" aria-label="Alternar tema">
        <img :src="bookIcon" class="action-button-img" alt="Moon icon" aria-hidden="true" />
      </button>
      <button class="action-button" type="button" aria-label="Alternar tema">
        <img :src="githubIcon" class="action-button-img" alt="Moon icon" aria-hidden="true" />
      </button>
    </footer>
  </main>
</template>

<style scoped>
@reference "../assets/main.css";

:global(html), :global(body), :global(#app) { @apply h-full overflow-hidden; }

.page-shell { @apply relative flex h-dvh max-h-dvh min-h-0 flex-col overflow-hidden box-border rounded-b-[20px] bg-white font-roboto text-[#242424] max-[760px]:px-3.5; }
.page-shell :deep(.app-header) { @apply shrink-0; }
.workspace { @apply flex min-h-0 flex-1 flex-col pt-[43px] pb-[97px] max-[760px]:pt-7 dark:bg-gunmetal; }
.controls-row { @apply flex shrink-0 items-end gap-3.5 max-[760px]:flex-wrap max-[760px]:items-stretch; }
.field { @apply flex flex-col gap-[7px]; }
.field > span, .steps-section h1, .result-row h2 { @apply text-[19px] tracking-[.1px] uppercase; }
input, select, .result-value { @apply box-border h-[45px] rounded-[10px] border-[1.5px] border-[#242424] bg-light-cyan px-[13px] text-[18px] leading-[1.2] text-[#242424] outline-none focus:outline-2 focus:outline-offset-2 focus:outline-turquoise; }
.expression-field { @apply w-[367px] max-[760px]:w-full; }
.notation-control { @apply flex items-end gap-[13px] max-[760px]:w-full max-[760px]:flex-wrap max-[760px]:items-stretch; }
.notation-field { @apply relative max-[760px]:flex-1 after:pointer-events-none after:absolute after:right-[14px] after:bottom-[18px] after:border-x-[5px] after:border-t-[7px] after:border-x-transparent after:border-t-[#242424]; }
.notation-field select { @apply w-41 appearance-none bg-light-cyan pr-10 text-[#242424] disabled:opacity-100 max-[760px]:w-full; }
.notation-field select option { @apply bg-light-cyan text-[#242424]; }
.action-button, .page-footer button { @apply flex h-[45px] w-[45px] items-center justify-center rounded-[10px] border-[1.5px] border-[#242424] bg-turquoise p-0 text-[27px] leading-none font-bold hover:-translate-y-px hover:brightness-[.96] focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-turquoise cursor-pointer; }
.action-button-img { @apply w-7 h-7}
.ufal-button-img { @apply w-10 h-10 cursor-pointer }
.font-text { @apply font-black dark:text-light-cyan }
.steps-section { @apply mt-7 flex min-h-0 flex-1 flex-col; }
.steps-card { @apply mt-[7px] min-h-0 flex-1 overflow-x-hidden overflow-y-auto box-border rounded-[23px] border-[1.5px] border-[#242424] bg-light-cyan px-8 py-[27px] max-[760px]:px-[18px] max-[760px]:py-[23px]; }
.step + .step { @apply mt-8; }
.step h2 { @apply mb-[6px] text-4xl leading-[1.25] max-[760px]:text-[22px]; }
.step-expression, .step-description { @apply mb-4 text-3xl leading-[1.45] max-[760px]:text-[14px] max-[760px]:[overflow-wrap:anywhere]; }
.step-expression { @apply font-inter; }
.step-description { @apply ml-3; }
.result-row { @apply mt-7 flex shrink-0 items-center gap-[7px] max-[760px]:flex-wrap max-[760px]:items-start; }
.result-row h2 { @apply whitespace-nowrap; }
.result-value { @apply flex h-[31px] min-w-[192px] items-center text-[13px]; }
.copy-button { @apply relative flex h-7 w-[25px] items-center justify-center border-0 bg-transparent p-0 text-[22px] hover:-translate-y-px focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-turquoise cursor-pointer; }
.copy-tooltip { @apply absolute -bottom-[31px] -left-3 hidden whitespace-nowrap rounded bg-[#242424] px-[6px] py-1 font-sans text-[12px] text-white; }
.copy-button:hover .copy-tooltip, .copy-button:focus-visible .copy-tooltip { @apply block; }
.page-footer { @apply absolute -right-px -bottom-px flex min-h-[72px] items-center gap-7 rounded-tl-[19px] rounded-br-[19px] border-[1.5px] border-[#242424] border-r-0 border-b-0 bg-white py-0 pr-[27px] pl-4 max-[760px]:gap-2.5 max-[760px]:px-3; }
.page-footer strong { @apply text-[36px] leading-none font-bold; }
.page-footer button { @apply w-[59px]; }
</style>

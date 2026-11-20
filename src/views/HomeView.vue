<script setup lang="ts">
import HeaderComponent from '../components/HeaderComponent.vue';
import { computed, Ref, ref, watch } from 'vue';
import { useConfigStore } from '../stores/config.ts';
import { PhCheckFat, PhCopy, PhEraser, PhLightbulbFilament, PhMagicWand,
  PhShuffleAngular } from '@phosphor-icons/vue';
import { useSimplificationStore } from '../stores/simplification.ts';
import { writeText } from '@tauri-apps/plugin-clipboard-manager';
import { invoke } from '@tauri-apps/api/core';
import type {
  ManualCompletion, ManualStepValidation, TruthTableRows,
} from '../utils/types.ts';
import Input from '@/components/ui/Input.vue';
import ButtonGroup from '@/components/ui/ButtonGroup.vue';
import Button from '@/components/ui/Button.vue';
import Card from '@/components/ui/Card.vue';
import CardHeader from '@/components/ui/CardHeader.vue';
import CardTitle from '@/components/ui/CardTitle.vue';
import CardContent from '@/components/ui/CardContent.vue';
import Checkbox from '@/components/ui/Checkbox.vue';
import Label from '@/components/ui/Label.vue';
import Tabs from '@/components/ui/Tabs.vue';
import TabsList from '@/components/ui/TabsList.vue';
import TabsTrigger from '@/components/ui/TabsTrigger.vue';
import TruthTable from '@/components/TruthTable.vue';
import PracticeComponent from '@/components/PracticeComponent.vue';
import SolutionComponent from '@/components/SolutionComponent.vue';
import { ActiveTab } from '@/utils/shared.ts';

const configStore = useConfigStore();
const simplificationStore = useSimplificationStore();

const copyButtonTittle = ref('COPIAR');

const isVerifying = ref(false);
const verificationMessage = ref('');
const verificationError = ref('');
const stepVerificationResults = ref<boolean[]>([]);
const hintMessage = ref('');
const verificationFeedback = computed(() => hintMessage.value
  || verificationError.value || verificationMessage.value);
let verificationVersion = 0;

const columns: Ref<string[]> = ref([]);
const rows: Ref<TruthTableRows[]> = ref([]);


watch(() => simplificationStore.steps.map(step => [step.value, step.law]), () => {
  verificationVersion++;
  isVerifying.value = false;
  verificationMessage.value = '';
  verificationError.value = '';
  stepVerificationResults.value = [];
  hintMessage.value = '';
}, { deep: true, flush: 'sync' });

async function copyResult() {
  await writeText(simplificationStore.result);
  copyButtonTittle.value = 'COPIADO!';
  setTimeout(() => {
    copyButtonTittle.value = 'COPIAR';
  }, 1000);
}

async function verifyManualExpression(requestHint = false) {
  if (isVerifying.value) return;
  verificationMessage.value = '';
  verificationError.value = '';
  hintMessage.value = '';

  const steps = simplificationStore.steps;
  if (steps.length < 2) {
    verificationError.value = 'Passos insuficientes.';
    if (requestHint) {
      hintMessage.value =
        'Adicione a expressão original e uma tentativa de simplificação.';
    }
    return;
  }

  const original = steps[0].value;
  const lastStep = steps[steps.length - 1];
  const candidate = lastStep.value;
  if (!original.trim() || !candidate.trim()) {
    verificationError.value = 'Há expressões vazias.';
    if (requestHint) hintMessage.value = 'Informe as duas expressões antes de verificar.';
    return;
  }

  const version = ++verificationVersion;
  isVerifying.value = true;
  try {
    const validations = await Promise.all(
      steps.slice(0, -1).map((step, index) =>
        invoke<ManualStepValidation>('verify_manual_step', {
          original,
          previous: step.value,
          candidate: steps[index + 1].value,
          selectedLaw: step.law,
        }),
      ),
    );
    if (version !== verificationVersion) return;

    stepVerificationResults.value = steps.map((step, index) =>
      (index === 0 || validations[index - 1].equivalence.equivalent)
      && (index === validations.length
        ? step.law === 'final' : validations[index].law_valid),
    );
    const firstIncorrectIndex = validations.findIndex(validation =>
      !validation.equivalence.equivalent || !validation.law_valid,
    );
    if (firstIncorrectIndex !== -1) {
      const validation = validations[firstIncorrectIndex];
      const stepLabel = `Passo ${firstIncorrectIndex + 1}: `;
      verificationError.value = stepLabel + (validation.equivalence.equivalent
        ? 'lei incorreta.' : 'expressão incorreta.');
      if (requestHint) hintMessage.value = stepLabel + validation.hint;
      return;
    }
    if (lastStep.law !== 'final') {
      const stepLabel = `Passo ${steps.length}: `;
      verificationError.value = stepLabel + 'conclusão pendente.';
      if (requestHint) {
        hintMessage.value = stepLabel
          + 'Selecione a lei FINAL para concluir a simplificação.';
      }
      return;
    }

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
        hintMessage.value = completion.next_law
          ? `Tentativa incompleta. Continue com a lei ${completion.next_law}.`
          : 'Os passos estão corretos, '
          + 'mas a última expressão ainda pode ser simplificada.';
      }
    }
  } catch (error) {
    if (version !== verificationVersion) return;
    verificationError.value = 'Não foi possível verificar as expressões.';
    if (requestHint) {
      const detail = error instanceof Error ? error.message : error;
      if (typeof detail === 'string') hintMessage.value = detail;
    }
  } finally {
    if (version === verificationVersion) isVerifying.value = false;
  }
}

watch([() => simplificationStore.expression, () => configStore.activeTab,
  () => configStore.compactTable], async () => {
  [columns.value, rows.value] =
    await simplificationStore.getTruthTable(simplificationStore.expression, columns,
      rows, configStore.compactTable);
  columns.value[columns.value.length - 1] = 'S';
});
</script>

<template>
  <main class="page-shell bg-background" :class="{ dark: configStore.darkTheme }">
    <HeaderComponent :is-manual-view="false" />
    <section class="workspace" aria-label="Simplificação de expressão booleana">
      <div class="flex min-h-0 gap-4">
        <div class="flex flex-col gap-4 flex-1">
          <Card>
            <CardHeader class="bg-primary">
              <CardTitle class="text-background-light">
                Expressão Booleana
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 flex flex-col gap-4">
              <Input
                placeholder="Escreva a expressão original" class="h-20 min-h-20
              placeholder:text-background-inverted text-2xl md:text-2xl font-semibold
                placeholder:text-lg text-background-inverted"
                v-model="simplificationStore.expression"
              />

              <ButtonGroup class="flex gap-2">
                <Button
                  variant="default" animation="pop"
                  class="grow-3 hover:cursor-pointer"
                  :disabled="simplificationStore.isSimplifying"
                  @click="simplificationStore.simplification"
                >
                  <PhMagicWand weight="fill" />
                  Simplificar
                </Button>
                <Button
                  variant="secondary" animation="pop"
                  class="grow hover:cursor-pointer"
                  @click="simplificationStore.randomExpressionHelper"
                >
                  <PhShuffleAngular weight="bold" />
                  Exemplo
                </Button>
                <Button
                  variant="accent" animation="pop"
                  class="grow hover:cursor-pointer"
                  @click="simplificationStore.clearData"
                >
                  <PhEraser weight="fill" />
                  Limpar
                </Button>
              </ButtonGroup>
            </CardContent>
          </Card>

          <Card>
            <CardHeader class="bg-primary flex flex-row justify-between">
              <CardTitle class="text-background-light">
                Resultado
              </CardTitle>
              <CardTitle class="text-background-light">
                {{ simplificationStore.stepsAutomatic.length }} passos
              </CardTitle>
            </CardHeader>
            <CardContent class="pt-6 flex justify-between items-center">
              <p class="text-2xl font-bold text-background-inverted">
                {{ simplificationStore.result }}
              </p>
              <Button
                variant="accent" animation="pop"
                class="hover:cursor-pointer" @click="copyResult"
              >
                <PhCopy weight="fill" />
                {{ copyButtonTittle }}
              </Button>
            </CardContent>
          </Card>
        </div>

        <Card class="flex-1 flex flex-col min-h-0 min-w-0">
          <CardHeader
            class="bg-primary shrink-0 flex flex-row justify-between
            items-center"
          >
            <div class="flex justify-between w-full">
              <Tabs v-model="configStore.activeTab">
                <TabsList>
                  <TabsTrigger
                    :value="ActiveTab.Solution" class="text-xs
                      data-[state=active]:bg-secondary data-[state=active]:text-black
                      hover:cursor-pointer"
                  >
                    Solução
                  </TabsTrigger>
                  <TabsTrigger
                    :value="ActiveTab.Practice" class="text-xs 
                      data-[state=active]:bg-secondary data-[state=active]:text-black
                      hover:cursor-pointer"
                  >
                    Praticar
                  </TabsTrigger>
                  <TabsTrigger
                    :value="ActiveTab.TruthTable" class="text-xs
                      data-[state=active]:bg-secondary data-[state=active]:text-black
                      hover:cursor-pointer"
                  >
                    Tabela Verdade
                  </TabsTrigger>
                </TabsList>
              </Tabs>

              <div
                class="flex items-center justify-end gap-2 w-full"
                v-if="configStore.activeTab === ActiveTab.Solution"
              >
                <Checkbox
                  id="controlled" v-model:checked="configStore.showExamples"
                  class="bg-secondary data-[state=checked]:bg-secondary
                  data-[state=checked]:text-black hover:cursor-pointer"
                />
                <Label for="controlled" class="text-background-light text-sm font-bold">
                  Exibir exemplos
                </Label>
              </div>

              <div
                class="flex items-center justify-end gap-2 w-full"
                v-else-if="configStore.activeTab === ActiveTab.Practice"
              >
                <Button
                  class="bg-accent text-black" animation="pop"
                  :disabled="isVerifying" @click="verifyManualExpression(true)"
                >
                  <PhLightbulbFilament :size="32" weight="fill" />
                  <p>Dica</p>
                </Button>
                <Button
                  class="bg-secondary text-black" animation="pop"
                  :disabled="isVerifying" @click="verifyManualExpression()"
                >
                  <PhCheckFat :size="32" weight="fill" />
                  <p>Verificar</p>
                </Button>
              </div>

              <div class="flex items-center justify-end gap-2 w-full" v-else>
                <Checkbox
                  id="controlled" v-model:checked="configStore.compactTable"
                  class="bg-secondary data-[state=checked]:bg-secondary
                  data-[state=checked]:text-black hover:cursor-pointer"
                />
                <Label for="controlled" class="text-background-light text-sm font-bold">
                  Tabela compacta
                </Label>
              </div>
            </div>
          </CardHeader>
          <CardContent class="flex flex-col flex-1 min-h-0">
            <SolutionComponent v-if="configStore.activeTab === ActiveTab.Solution" />

            <div v-else-if="configStore.activeTab === ActiveTab.Practice">
              <p
                v-if="verificationFeedback"
                class="mb-2 text-lg" role="status"
              >
                {{ verificationFeedback }}
              </p>
              <PracticeComponent :verification-results="stepVerificationResults" />
            </div>

            <TruthTable
              v-else :columns="columns"
              :rows="rows"
            />
          </CardContent>
        </Card>
      </div>
    </section>
  </main>
</template>

<style scoped>
@reference "../assets/main.css";

.page-shell {
  @apply relative flex h-dvh max-h-dvh min-h-0 flex-col overflow-hidden box-border rounded-b-[20px] font-roboto text-[#242424] max-[760px]:px-3.5;
}

.page-shell :deep(.app-header) {
  @apply shrink-0;
}

.page-shell :deep(button[role="combobox"]) {
  animation: none;
}

.workspace {
  @apply grid min-h-0 flex-1 grid-rows-[minmax(0,1fr)] px-8;
  @apply pt-[43px] pb-7 max-[760px]:pt-7;
}
</style>

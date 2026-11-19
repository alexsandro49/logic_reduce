import { defineStore } from 'pinia';
import { Ref, ref } from 'vue';
import { ReductionStep, SolutionStep, StepStruct, TruthTable, TruthTableRows } 
  from '../utils/types';
import { generateRandomExpression } from '../utils/expressionGenerator';
import { invoke } from '@tauri-apps/api/core';
import { useConfigStore } from './config';
import { ActiveTab } from '@/utils/shared';

export const useSimplificationStore = defineStore(
  'simplification',
  () => {
    const expression = ref('');
    const placeholder = '~((~A + B) & (~B + C))';

    const isSimplifying = ref(false);
    const error = ref('');
    // const steps: Ref<ReductionStep[]> = ref([]);
    const steps: Ref<ReductionStep[]> = ref([
      { value: '~((~A + B) & (~B + C))', law: 'commutative' },
      { value: '~((B + ~A) & (~B + C))', law: 'commutative' },
      { value: '~((B + ~A) & (C + ~B))', law: 'de_morgan' },
      { value: '~(B + ~A) + ~(C + ~B)', law: 'de_morgan' },
      { value: '(~B & ~~A) + ~(C + ~B)', law: 'double_negative' },
      { value: '(~B & A) + ~(C + ~B)', law: 'commutative' },
      { value: '(A & ~B) + ~(C + ~B)', law: 'de_morgan' },
      { value: '(A & ~B) + (~C & ~~B)', law: 'double_negative' },
      { value: '(A & ~B) + (~C & B)', law: 'commutative' },
      { value: '(A & ~B) + (B & ~C)', law: 'final' }
    ]);
    const result = ref('');
    const submittedExpression = ref('');

    const stepsAutomatic: Ref<SolutionStep[]> = ref([]);

    async function simplification() {
      const source = expression.value.trim() || placeholder;

      isSimplifying.value = true;
      error.value = '';
      submittedExpression.value = source;
      stepsAutomatic.value = [];
      result.value = '';
      expression.value = source;

      try {
        const [simplifiedExpression, newSteps] = await invoke<[string, StepStruct[]]>(
          'simplify_helper',
          { source },
        );

        result.value = simplifiedExpression;


        let previousState = '';
        stepsAutomatic.value = newSteps.map((step, index) => {
          const newStep = {
            name: step.law_name,
            before: index === 0 ? submittedExpression.value : previousState,
            after: step.after_law
          };

          previousState = step.after_law;

          return newStep;
        });
      } catch (reason) {
        error.value = typeof reason === 'string' ? reason :
          'Não foi possível simplificar a expressão.';
      } finally {
        isSimplifying.value = false;
      }
    }

    function randomExpressionHelper() {
      clearData();
      expression.value =
        generateRandomExpression(expression.value);
      simplification();
    }

    function clearData() {
      isSimplifying.value = false;
      error.value = '';
      steps.value = [];
      stepsAutomatic.value = [];
      result.value = '';
      expression.value = '';

      const configStore = useConfigStore();
      configStore.activeTab = ActiveTab.Solution;
    }

    function addStep() {
      const value = expression.value.trim() || placeholder;

      steps.value.push({ value: value, law: 'identity' });
      expression.value = '';
    }

    function removeStep(indexToRemove: number) {
      steps.value = steps.value.filter((_, index) => index !== indexToRemove);
    }

    async function getTruthTable(value: string, columns: Ref<string[]>,
      rows: Ref<TruthTableRows[]>, compactTable: boolean): 
      Promise<[string[], TruthTableRows[]]> {
        
      if (value == '') {
        value = placeholder;
        expression.value = value;
      }

      try {
        const table = await invoke<TruthTable>('truth_table_helper', {
          source: value,
          display: compactTable ? 'reduced' : 'complete',
          includeIntermediate: true,
        });

        columns.value = [...table.inputs, ...table.columns];
        rows.value = table.rows.map((row) => [...row.inputs, ...row.outputs]);

        return [columns.value, rows.value];
      } catch (error) {
        throw new Error('Erro ao gerar tabela', { cause: error });
      }
    }

    return {
      expression,
      placeholder,
      isSimplifying,
      simplification,
      steps,
      submittedExpression,
      randomExpressionHelper,
      clearData,
      addStep,
      removeStep,
      stepsAutomatic,
      error,
      result,
      getTruthTable
    };
  }
);

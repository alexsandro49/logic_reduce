import { defineStore } from 'pinia';
import { Ref, ref } from 'vue';
import { ReductionStep, ReductionStepAutomatic, StepStruct } from '../utils/types';
import { generateRandomExpression } from '../utils/expressionGenerator';
import { invoke } from '@tauri-apps/api/core';

export const useSimplificationStore = defineStore(
  'simplification',
  () => {
    const expression = ref('');
    const placeholder = '~((~A + B) & (~B + C))';

    const isSimplifying = ref(false);
    const error = ref('');
    const steps: Ref<ReductionStep[]> = ref([]);
    const result = ref('');
    const submittedExpression = ref('');

    const stepsAutomatic: Ref<ReductionStepAutomatic[]> = ref([]);

    async function simplification() {
      const source = expression.value.trim() || placeholder;

      isSimplifying.value = true;
      error.value = '';
      submittedExpression.value = source;
      stepsAutomatic.value = [];
      result.value = '';

      try {
        const [simplifiedExpression, newSteps] = await invoke<[string, StepStruct[]]>(
          'simplify_helper',
          { source },
        );

        result.value = simplifiedExpression;
        stepsAutomatic.value = newSteps.map((step) => ({
          title: step.rule_name,
          expression: step.after_rule,
          description: 'Regra aplicada durante a simplificação.',
        }));
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
    }

    function addStep() {
      const value = expression.value.trim() || placeholder;

      steps.value.push({value: value, rule: 'identity'});
      expression.value = '';
    }

    function removeStep(indexToRemove: number) {
      steps.value = steps.value.filter((_, index) => index !== indexToRemove);
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
      result
    };
  }
);

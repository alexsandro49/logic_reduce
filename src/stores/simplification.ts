import { defineStore } from 'pinia';
import { Ref, ref } from 'vue';
import { ReductionStep } from '../utils/types';
// import { invoke } from '@tauri-apps/api/core';
import { generateRandomExpression } from '../utils/expressionGenerator';

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

    // async function simplification() {
    //   const source = expression.value.trim() || placeholder;

    //   isSimplifying.value = true;
    //   error.value = '';
    //   submittedExpression.value = source;
    //   steps.value = [];
    //   result.value = '';

      // try {
      //   const [simplifiedExpression, newSteps] = await invoke<[string, stepStruct[]]>(
      //     'simplify_helper',
      //     { source, notation: 'default' },
      //   );

      //   result.value = simplifiedExpression;
      //   steps.value = newSteps.map((step) => ({
      //     title: step.rule_name,
      //     expression: step.after_rule,
      //     description: 'Regra aplicada durante a simplificação.',
      //   }));
      // } catch (reason) {
      //   error.value = typeof reason === 'string' ? reason :
      //     'Não foi possível simplificar a expressão.';
      // } finally {
      //   isSimplifying.value = false;
      // }
    // }

    function randomExpressionHelper() {
      clearData();
      expression.value =
        generateRandomExpression(expression.value);
      // simplification();
    }

    function clearData() {
      isSimplifying.value = false;
      error.value = '';
      steps.value = [];
      result.value = '';
      expression.value = '';
    }

    function addStep() {
      const value = expression.value.trim() || placeholder;

      steps.value.push({value: value, rule: 'identity'})
      expression.value = '';
    }

    function removeStep(indexToRemove: number) {
      steps.value = steps.value.filter((_, index) => index !== indexToRemove);
    }

    return {
      expression,
      placeholder,
      isSimplifying,
      // simplification,
      steps,
      submittedExpression,
      randomExpressionHelper,
      clearData,
      addStep,
      removeStep
    };
  }
);

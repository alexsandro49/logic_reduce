import { defineStore } from 'pinia';
import { ref } from 'vue';

export const useSimplificationStore = defineStore(
  'simplification',
  () => {
    const expression = ref('');
    const placeholder = '~((~A + B) & (~B + C))';

    const isSimplifying = ref(false);

    return {
      expression,
      placeholder,
      isSimplifying
    };
  }
);

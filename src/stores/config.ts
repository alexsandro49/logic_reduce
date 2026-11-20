import { defineStore } from 'pinia';
import { ref } from 'vue';
import { ActiveTab } from '@/utils/shared';

export const useConfigStore = defineStore(
  'config',
  () => {
    const darkTheme = ref(false);
    const currentTheme = ref(0);

    const activeTab = ref(ActiveTab.Solution);
    const showExamples = ref(false);
    const compactTable = ref(false);

    return {
      darkTheme,
      currentTheme,
      activeTab,
      showExamples,
      compactTable
    };
  },
  {
    persist: {
      omit: ['activeTab']
    },
  },
);
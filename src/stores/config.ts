import { defineStore } from "pinia";
import { ref } from "vue";

export const useConfigStore = defineStore(
  "config",
  () => {
    const darkTheme = ref(false);
    
    return {
      darkTheme,

    };
  },
  {
    persist: true,
  },
);
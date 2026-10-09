import { defineStore } from "pinia";
import { computed, ref } from "vue";
import { Color } from "../utils/types";

export const useConfigStore = defineStore(
  "config",
  () => {
    const darkTheme = ref(false);
    const currentTheme = ref(0);

    const themes: Color[][] = [
      [
        { color: '#11999E', uses: ['base'] },
        { color: '#E4F9F5', uses: ['board', 'result-value', 'dark-text', 'input', 'navigation-link'] },
        { color: '#30E3CA', uses: ['action-button', 'hightlighted-cell', 'footer-buttons-background',
                                    'header-active-tab'] },
        { color: '#40514E', uses: [] },
        { color: '#343A40', uses: ['text', 'input-text', 'header-table', 'background', 'dark-cell-text', 
                                    'light-cell-text', 'icon-color', 'footer-buttons-color'] },
        { color: '#00000000', uses: ['transparent', 'header-tab'] },
      ],
      [
        { color: '#071A52', uses: ['base'] },
        { color: '#E4FBFF', uses: ['board', 'result-value', 'input'] },
        { color: '#A7FF83', uses: ['action-button', 'dark-text', 'footer-buttons-background',
                                    'header-active-tab'] },
        { color: '#086972', uses: [] },
        { color: '#17B978', uses: ['navigation-link', 'hightlighted-cell', 'header-tab'] },
        { color: '#343A40', uses: ['text', 'input-text', 'header-table', 'background', 
                                    'dark-cell-text', 'light-cell-text', 'icon-color', 
                                    'footer-buttons-color'] },
        { color: '#00000000', uses: ['transparent'] },
      ],
      [
        { color: '#222831', uses: ['base', 'text', 'input-text', 'header-table', 'dark-cell-text', 
                                    'icon-color'] },
        { color: '#30475E', uses: ['background', 'hightlighted-cell', 'footer-buttons-color'] },
        { color: '#F05454', uses: ['action-button', 'footer-buttons', 'footer-buttons-background', 
                                    'header-active-tab'] },
        { color: '#DDDDDD', uses: ['board', 'input', 'navigation-link', 'result-value', 'dark-text',
                                    'light-cell-text', 'header-tab'] },
        { color: '#00000000', uses: ['transparent'] },
      ],
      [
        { color: '#7868E6', uses: ['base', 'action-button', 'header-table', 'dark-cell-text', 
                                    'footer-buttons-background'] },
        { color: '#B8B5FF', uses: ['hightlighted-cell', 'navigation-link', 'header-tab', 
                                    'dark-text'] },
        { color: '#EDEEF7', uses: ['header-active-tab'] },
        { color: '#E4FBFF', uses: ['board', 'input', 'result-value', 'light-cell-text'] },
        { color: '#343A40', uses: ['background', 'text', 'icon-color', 'footer-buttons-color', 
                                    'input-text'] },
        { color: '#00000000', uses: ['transparent'] },
      ],
      [
        { color: '#850E35', uses: ['base', 'header-table', 'dark-cell-text'] },
        { color: '#EE6983', uses: ['header-active-tab', 'hightlighted-cell', 'dark-text', 'action-button',
                                    'footer-buttons-background'] },
        { color: '#FFC4C4', uses: ['header-tab', 'navigation-link'] },
        { color: '#FFF5E4', uses: ['board', 'input', 'result-value', 'light-cell-text'] },
        { color: '#343A40', uses: ['background', 'text', 'icon-color', 'footer-buttons-color', 
                                    'input-text'] },
        { color: '#00000000', uses: ['transparent'] },
      ]
    ]

    const themeColors = computed(() => themes[currentTheme.value]);
    const whiteLogo = computed(() => (currentTheme.value == 0) ? 0 : 1);

    function nextTheme() {
      currentTheme.value = (currentTheme.value + 1) % themes.length;
    }

    function getColor(use: string): string {
      const variant = (darkTheme.value && use == 'text') ? 'dark-' : '';
      console.log(`${variant}${use}`)
      return themeColors.value.find((color: Color) => color.uses.includes(`${variant}${use}`))!.color
    }

    return {
      darkTheme,
      nextTheme,
      themeColors,
      whiteLogo,
      getColor
    };
  },
  {
    persist: true,
  },
);
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
        { color: '#E4F9F5', uses: ['board', 'header-tab', 'result-value', 'dark-text', 'input', 'navigation-link'] },
        { color: '#30E3CA', uses: ['action-button', 'hightlighted-cell', 'footer-buttons-background',
                                    'header-active-tab'] },
        { color: '#40514E', uses: [] },
        { color: '#343A40', uses: ['text', 'input-text', 'header-table', 'background', 'dark-cell-text', 
                                    'light-cell-text', 'icon-color', 'footer-buttons-color'] },
        { color: '#00000000', uses: ['transparent'] },
      ],
      [
        { color: '#071A52', uses: ['base'] },
        { color: '#E4FBFF', uses: ['board', 'result-value', 'input', 'dark-text'] },
        { color: '#A7FF83', uses: ['action-button', 'footer-buttons-background',
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
        { color: '#7868E6', uses: ['base', 'action-button', 'dark-cell-text', 
                                    'footer-buttons-background', 'hightlighted-cell'] },
        { color: '#B8B5FF', uses: ['navigation-link', 'header-tab', 
                                    'dark-text'] },
        { color: '#EDEEF7', uses: ['board', 'input'] },
        { color: '#E4FBFF', uses: ['header-active-tab', 'result-value', 'light-cell-text'] },
        { color: '#343A40', uses: ['background', 'text', 'icon-color', 'footer-buttons-color', 
                                    'input-text', 'header-table'] },
        { color: '#00000000', uses: ['transparent'] },
      ],
      [
        { color: '#f8639a', uses: ['base', 'action-button', 'footer-buttons-background'] },
        { color: '#e9c0e8', uses: ['header-active-tab'] },
        { color: '#bbd4f2', uses: ['header-tab', 'navigation-link', 'hightlighted-cell'] },
        { color: '#dddddd', uses: ['board', 'dark-text', 'input', 'result-value'] },
        { color: '#343A40', uses: ['background', 'text', 'icon-color', 'footer-buttons-color',
                                    'input-text', 'dark-cell-text', 'light-cell-text', 'header-table'] },
        { color: '#00000000', uses: ['transparent'] },
      ],
      [
        { color: '#222831', uses: ['base', 'header-table', 'dark-cell-text', 'icon-color',
                                    'text', 'input-text', 'footer-buttons-color'] },
        { color: '#393E46', uses: ['hightlighted-cell', 'background'] },
        { color: '#FFD369', uses: ['header-active-tab', 'navigation-link', 'action-button',
                                    'footer-buttons-background'] },
        { color: '#EEEEEE', uses: ['board', 'header-tab', 'input', 'result-value', 'light-cell-text', 'dark-text'] },
        { color: '#00000000', uses: ['transparent'] },
      ]
    ]

    const themeColors = computed(() => themes[currentTheme.value]);

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
      getColor
    };
  },
  {
    persist: true,
  },
);
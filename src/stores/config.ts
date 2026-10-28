import { defineStore } from 'pinia';
import { computed, ref } from 'vue';
import { Color } from '../utils/types';

export const useConfigStore = defineStore(
  'config',
  () => {
    const darkTheme = ref(false);
    const currentTheme = ref(0);

    const themes: Color[][] = [
      [
        { color: '#11999E', tags: ['base'] },
        { color: '#E4F9F5', tags: ['board', 'header-tab', 'dark-text', 'input',
          'navigation-link'] },
        {
          color: '#30E3CA', tags: ['action-button', 'hightlighted-cell',
            'footer-buttons-background', 'header-active-tab']
        },
        {
          color: '#343A40', tags: ['text', 'input-text', 'header-table', 'background',
            'dark-cell-text', 'light-cell-text', 'icon-color', 'footer-buttons-color']
        },
        { color: '#00000000', tags: ['transparent'] },
        { color: '#f05454', tags: ['error'] },
      ],
      [
        { color: '#071A52', tags: ['base'] },
        { color: '#E4FBFF', tags: ['board', 'input', 'dark-text'] },
        {
          color: '#A7FF83', tags: ['action-button', 'footer-buttons-background',
            'header-active-tab']
        },
        { color: '#17B978', tags: ['navigation-link', 'hightlighted-cell', 'header-tab']},
        {
          color: '#343A40', tags: ['text', 'input-text', 'header-table', 'background',
            'dark-cell-text', 'light-cell-text', 'icon-color', 'footer-buttons-color']
        },
        { color: '#00000000', tags: ['transparent'] },
        { color: '#f05454', tags: ['error'] },
      ],
      [
        { color: '#136b3d', tags: ['base', 'dark-cell-text', 'hightlighted-cell'] },
        {
          color: '#9ed880', tags: ['navigation-link', 'header-tab', 'action-button',
            'footer-buttons-background']
        },
        { color: '#df8c22', tags: ['header-active-tab'] },
        { color: '#dddddd', tags: ['board', 'input', 'light-cell-text', 'dark-text'] },
        {
          color: '#0c2132', tags: ['background', 'text', 'icon-color',
            'footer-buttons-color', 'input-text', 'header-table']
        },
        { color: '#00000000', tags: ['transparent'] },
        { color: '#f05454', tags: ['error'] },
      ],
      [
        { color: '#f8639a', tags: ['base', 'action-button', 'footer-buttons-background']},
        { color: '#e9c0e8', tags: ['header-active-tab'] },
        { color: '#bbd4f2', tags: ['header-tab', 'navigation-link', 'hightlighted-cell']},
        { color: '#dddddd', tags: ['board', 'dark-text', 'input'] },
        {
          color: '#343A40', tags: ['background', 'text', 'icon-color',
            'footer-buttons-color', 'input-text', 'dark-cell-text', 'light-cell-text',
            'header-table']
        },
        { color: '#00000000', tags: ['transparent'] },
        { color: '#f05454', tags: ['error'] },
      ],
      [
        {
          color: '#222831', tags: ['base', 'text', 'input-text', 'header-table',
            'dark-cell-text', 'icon-color']
        },
        { color: '#30475E', tags: ['background', 'hightlighted-cell',
          'footer-buttons-color'] },
        {
          color: '#F05454', tags: ['action-button', 'footer-buttons',
            'footer-buttons-background', 'header-active-tab']
        },
        {
          color: '#DDDDDD', tags: ['board', 'input', 'navigation-link', 'dark-text',
            'light-cell-text', 'header-tab']
        },
        { color: '#00000000', tags: ['transparent'] },
        { color: '#f05454', tags: ['error'] },
      ],
      [
        {
          color: '#222831', tags: ['base', 'header-table', 'dark-cell-text', 'icon-color',
            'text', 'input-text', 'footer-buttons-color']
        },
        { color: '#393E46', tags: ['hightlighted-cell', 'background'] },
        {
          color: '#FFD369', tags: ['header-active-tab', 'footer-buttons-background',
            'navigation-link', 'action-button']
        },
        { color: '#EEEEEE', tags: ['board', 'header-tab', 'input', 'light-cell-text',
          'dark-text'] },
        { color: '#00000000', tags: ['transparent'] },
        { color: '#f05454', tags: ['error'] },
      ]
    ];

    const themeColors = computed(() => themes[currentTheme.value]);

    function nextTheme() {
      currentTheme.value = (currentTheme.value + 1) % themes.length;
    }

    function getColor(use: string): string {
      const variant = (darkTheme.value && use == 'text') ? 'dark-' : '';
      return themeColors.value.find((color: Color) => 
        color.tags.includes(`${variant}${use}`))!.color;
    }

    return {
      darkTheme,
      currentTheme,
      nextTheme,
      themeColors,
      getColor
    };
  },
  {
    persist: true,
  },
);
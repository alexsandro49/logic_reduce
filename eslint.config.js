import js from '@eslint/js';
import globals from 'globals';
import tseslint from 'typescript-eslint';
import pluginVue from 'eslint-plugin-vue';
import { defineConfig } from 'eslint/config';

export default defineConfig([
  { ignores: ['dist/**', 'src-tauri/target/**'] },
  { files: ['**/*.{js,mjs,cjs,ts,mts,cts,vue}'], plugins: { js }, 
    extends: ['js/recommended'], languageOptions: { 
      globals: {...globals.browser, ...globals.node} 
    } },
  tseslint.configs.recommended,
  pluginVue.configs['flat/strongly-recommended'],
  { files: ['**/*.vue'], languageOptions: { parserOptions: { parser: tseslint.parser } }},
  { files: ['**/*.{js,ts,vue}'],
    rules: {
      semi: ['error', 'always'],
      quotes: ['error', 'single'],
      indent: ['error', 2],
      'comma-spacing': ['error', {before: false, after: true}],
      'no-multi-spaces': 0,
      'vue/multi-word-component-names': 'off',
      'vue/max-len': ['error', {
        'code': 90,
        'template': 90,
        'tabWidth': 2,
        'comments': 90,
        'ignorePattern': '',
        'ignoreComments': false,
        'ignoreTrailingComments': false,
        'ignoreUrls': false,
        'ignoreStrings': false,
        'ignoreTemplateLiterals': false,
        'ignoreRegExpLiterals': false,
        'ignoreHTMLAttributeValues': false,
        'ignoreHTMLTextContents': false,
      }],
      'vue/max-attributes-per-line': ['error', {
        'singleline': {
          'max': 2
        },
        'multiline': {
          'max': 2
        }
      }]
    }
  }
]);

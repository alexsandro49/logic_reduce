<script setup lang="ts">
import { useRouter } from 'vue-router'
import { useConfigStore } from '../stores/config'
import {PhCircuitry, PhMoon, PhPalette, PhSun} from '@phosphor-icons/vue'

const props = defineProps<{
  activeWindowButton: number;
}>();

const windowButtons = [
  {name: "simplificação", route: ""},
  {name: "tabela verdade", route: "truth-table"},
  {name: "manual", route: "manual"}
];

const configStore = useConfigStore();

const router = useRouter();

function buttonColorHelper(value: number) {
  const tag = props.activeWindowButton == value ? 'header-active-tab' : 'header-tab';
  return configStore.getColor(tag);
}
</script>

<template>
  <header class="app-header" :style="{backgroundColor: configStore.getColor('base')}">
    <a class="brand" href="#" aria-label="Logic Reduce">
      <PhCircuitry :size="44" class="text-white" weight="fill" />
    </a>

    <nav v-for="(link, index) in windowButtons" v-key="index" class="main-navigation" aria-label="Navegação principal">
      <a :style="{backgroundColor: buttonColorHelper(index)}" class="navigation-link hover:-translate-y-px hover:brightness-[.96]" @click="router.push(`/${link.route}`)">{{ link.name }}</a>
    </nav>

    <div class="header-actions">
      <button @click="configStore.darkTheme = !configStore.darkTheme" class="icon-button hover:-translate-y-px hover:brightness-[.96]" :style="{backgroundColor: configStore.getColor('navigation-link')}" type="button" aria-label="Alternar tema">
        <PhSun v-if="configStore.darkTheme == true" :size="28" :color="configStore.getColor('icon-color')" weight="fill" />
        <PhMoon v-else :size="28" :color="configStore.getColor('icon-color')" weight="fill" />
        <span class="tooltip" :style="{backgroundColor: configStore.getColor('base')}">ALTERNAR MODO</span>
      </button>
      <button class="icon-button hover:-translate-y-px hover:brightness-[.96]" @click="configStore.nextTheme()" :style="{backgroundColor: configStore.getColor('navigation-link')}" type="button" aria-label="Imprimir página">
        <PhPalette :size="28" :color="configStore.getColor('icon-color')" weight="fill" />
        <span class="tooltip" :style="{backgroundColor: configStore.getColor('base')}">ALTERNAR TEMA</span>
      </button>
    </div>
  </header>
</template>

<style scoped>
@reference "../assets/main.css";

.app-header {
  @apply flex box-border min-h-[74px] w-full items-center gap-[14px] border-[1.5px] border-solid border-[#222] py-[13px] pr-[14px] pl-[18px] font-roboto;
}

.brand {
  @apply flex h-[38px] w-[38px] flex-[0_0_38px] items-center justify-center overflow-hidden;
}

.main-navigation {
  @apply flex flex-wrap items-center gap-[13px];
}

.navigation-link,
.icon-button {
  @apply border-[1.5px] border-solid border-[#222] font-roboto text-[1.1em] font-medium;
}

.navigation-link {
  @apply flex min-h-[44px] items-center rounded-[11px] bg-transparent px-[11px] py-0 font-extrabold whitespace-nowrap uppercase no-underline;
}

.navigation-link:hover,
.navigation-link:focus-visible,
.icon-button:hover,
.icon-button:focus-visible {
  @apply cursor-pointer;
}

.header-actions {
  @apply ml-auto flex gap-[13px];
}

.icon-button {
  @apply flex relative h-[44px] w-[45px] cursor-pointer items-center justify-center rounded-[11px] p-0;
}

.icon-button img {
  @apply h-[26px] w-[24px] fill-[#222] stroke-[#222] stroke-[1.8] [stroke-linecap:round] [stroke-linejoin:round];
}

.icon-button:hover .tooltip,
.icon-button:focus-visible .tooltip {
  @apply block;
}

.tooltip { 
  @apply absolute hidden whitespace-nowrap rounded px-[6px] py-1 font-sans text-[12px] leading-normal 
  font-black text-white -bottom-[31px] right-0; 
}

@media (max-width: 680px) {
  .app-header {
    @apply mx-[10px] mt-[12px] w-[calc(100%_-_20px)] flex-wrap items-start p-[12px];
  }

  .main-navigation {
    @apply flex-[1_1_calc(100%_-_56px)] gap-[8px];
  }

  .navigation-link {
    @apply min-h-[38px] px-[8px] py-0 text-[14px];
  }

  .header-actions {
    @apply order-3 ml-auto w-full;
  }

  .icon-button {
    @apply h-[38px] w-[40px];
  }

  .tooltip {
    @apply right-auto left-0;
  }
}
</style>

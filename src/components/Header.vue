<script setup lang="ts">
import logo from '@/assets/circuitry.svg'
import moonIcon from '@/assets/moon.svg'
import sunIcon from '@/assets/sun.svg'
import paintRollerIcon from '@/assets/paint-roller.svg'
import { useRouter } from 'vue-router'
import { useConfigStore } from '../stores/config'

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
</script>

<template>
  <header class="app-header">
    <a class="brand" href="#" aria-label="Logic Reduce">
      <img :src="logo" alt="Program logo" aria-hidden="true" />
    </a>

    <nav v-for="(link, index) in windowButtons" v-key="index" class="main-navigation" aria-label="Navegação principal">
      <a :class="{'is-active': activeWindowButton == index}" class="navigation-link" @click="router.push(`/${link.route}`)">{{ link.name }}</a>
    </nav>

    <div class="header-actions">
      <button v-if="configStore.darkTheme == true" @click="configStore.darkTheme = !configStore.darkTheme" class="icon-button" type="button" aria-label="Alternar tema">
        <img :src="sunIcon" alt="Moon icon" aria-hidden="true" />
      </button>
      <button v-else class="icon-button" @click="configStore.darkTheme = !configStore.darkTheme" type="button" aria-label="Alternar tema">
        <img :src="moonIcon" alt="Moon icon" aria-hidden="true" />
      </button>
      <button class="icon-button" type="button" aria-label="Imprimir página">
        <img :src="paintRollerIcon" alt="Sun icon" aria-hidden="true" />
      </button>
    </div>
  </header>
</template>

<style scoped>
.app-header {
  align-items: center;
  background: var(--color-dark-cyan);
  border: 1.5px solid #222;
  box-sizing: border-box;
  display: flex;
  gap: 14px;
  min-height: 74px;
  padding: 13px 14px 13px 18px;
  width: 100%;
  font-family: var(--font-roboto);
}

.brand {
  align-items: center;
  display: flex;
  flex: 0 0 38px;
  height: 38px;
  justify-content: center;
  overflow: hidden;
  width: 38px;
}

.brand img {
  display: block;
  height: 44px;
  width: 44px;
}

.main-navigation {
  align-items: center;
  display: flex;
  flex-wrap: wrap;
  gap: 13px;
}

.navigation-link,
.icon-button {
  border: 1.5px solid #222;
  color: #222;
  font-family: var(--font-roboto);
  font-size: 1.1em;
  font-weight: 500;
}

.navigation-link {
  align-items: center;
  background: transparent;
  border-radius: 11px;
  display: flex;
  min-height: 44px;
  padding: 0 11px;
  text-decoration: none;
  text-transform: uppercase;
  font-weight: 800;
  white-space: nowrap;
}

.navigation-link.is-active {
  background: var(--color-turquoise);
}

.navigation-link:hover,
.navigation-link:focus-visible,
.icon-button:hover,
.icon-button:focus-visible {
  outline: 2px solid var(--color-light-cyan);
  cursor: pointer;
}

.header-actions {
  display: flex;
  gap: 13px;
  margin-left: auto;
}

.icon-button {
  align-items: center;
  background: var(--color-turquoise);
  border-radius: 11px;
  cursor: pointer;
  display: flex;
  height: 44px;
  justify-content: center;
  padding: 0;
  width: 45px;
}

.icon-button img {
  fill: #222;
  height: 26px;
  stroke: #222;
  stroke-linecap: round;
  stroke-linejoin: round;
  stroke-width: 1.8;
  width: 24px;
}

@media (max-width: 680px) {
  .app-header {
    align-items: flex-start;
    flex-wrap: wrap;
    margin: 12px 10px 0;
    padding: 12px;
    width: calc(100% - 20px);
  }

  .main-navigation {
    flex: 1 1 calc(100% - 56px);
    gap: 8px;
  }

  .navigation-link {
    font-size: 14px;
    min-height: 38px;
    padding: 0 8px;
  }

  .header-actions {
    margin-left: auto;
    order: 3;
    width: 100%;
  }

  .icon-button {
    height: 38px;
    width: 40px;
  }
}
</style>

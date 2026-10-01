<script setup lang="ts">
import logo from '@/assets/circuitry.svg'
import moonIcon from '@/assets/moon.svg'
import paintRollerIcon from '@/assets/paint-roller.svg'
</script>

<template>
  <header class="app-header">
    <a class="brand" href="#" aria-label="Logic Reduce">
      <img :src="logo" alt="Program logo" aria-hidden="true" />
    </a>

    <nav class="main-navigation" aria-label="Navegação principal">
      <a class="navigation-link is-active" href="#simplificacao">Simplificação</a>
      <a class="navigation-link" href="#tabela-verdade">Tabela verdade</a>
      <a class="navigation-link" href="#manual">Manual</a>
    </nav>

    <div class="header-actions">
      <button class="icon-button" type="button" aria-label="Alternar tema">
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
  border-radius: 0px 0px 20px 20px;
  box-sizing: border-box;
  display: flex;
  gap: 14px;
  min-height: 74px;
  padding: 13px 14px 13px 18px;
  width: 100%;
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
  font: 400 18px/1.1 'Roboto Mono', 'Courier New', monospace;
}

.navigation-link {
  align-items: center;
  background: var(--color-turquoise);
  border-radius: 11px;
  display: flex;
  min-height: 44px;
  padding: 0 11px;
  text-decoration: none;
  text-transform: uppercase;
  white-space: nowrap;
}

.navigation-link.is-active {
  background: var(--color-light-cyan);
}

.navigation-link:hover,
.navigation-link:focus-visible,
.icon-button:hover,
.icon-button:focus-visible {
  outline: 2px solid var(--color-light-cyan);
  outline-offset: 2px;
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

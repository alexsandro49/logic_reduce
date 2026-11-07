<script setup lang="ts">
import { computed, ref } from 'vue';
import { openUrl } from '@tauri-apps/plugin-opener';
import HeaderComponent from '@/components/HeaderComponent.vue';
import { useConfigStore } from '../stores/config';
import { LawDetails } from '../utils/types';
import { lawsDetails } from '../utils/shared';

const configStore = useConfigStore();
const selectedName = ref('Identidade');
const selectedLaw = computed(() => lawsDetails.find((law) => 
  law.name === selectedName.value) ?? lawsDetails[0]);

async function openExternalLink(event: MouseEvent, url: string) {
  event.preventDefault();
  await openUrl(url);
};

function sidebarColorHelper(law: LawDetails) {
  const value = (selectedName.value === law.name) ? 'action-button' : 'transparent';
  return configStore.getColor(value);
}
</script>

<template>
  <main
    class="manual-page"
    :class="{ dark: configStore.darkTheme }"
  >
    <HeaderComponent :active-window-button="2" />
    <div
      class="manual-layout"
      :style="{ backgroundColor: configStore.darkTheme ?
        configStore.getColor('background') : '#FFFFFF' }"
    >
      <aside
        class="sidebar"
        :style="{ backgroundColor: configStore.getColor('board') }"
        aria-label="Regras da álgebra booleana"
      >
        <h1 :style="{ color: configStore.getColor('input-text') }">
          Propriedades
        </h1>
        <nav>
          <button
            v-for="law in lawsDetails"
            :key="law.name"
            class="law-link"
            :style="{ backgroundColor: sidebarColorHelper(law) }"
            :aria-current="selectedName === law.name ? 'page' : undefined"
            @click="selectedName = law.name"
          >
            {{ law.name }}
          </button>
        </nav>
        <div class="topic">
          <span :style="{color: configStore.getColor('input-text') }">Sobre o tema:</span>
          <a
            :style="{ color: configStore.getColor('board'),
                      backgroundColor: configStore.getColor('base') }"
            href="https://pt.wikipedia.org/wiki/%C3%81lgebra_booleana"
            @click="openExternalLink(
              $event,'https://pt.wikipedia.org/wiki/%C3%81lgebra_booleana')"
          >Álgebra
            Booleana</a>
        </div>
      </aside>

      <article class="content">
        <h2
          class="colored-text"
          :style="{ color: configStore.getColor('text') }"
        >
          {{ selectedLaw.name === 'D. Negação' ? 'Dupla Negação' : selectedLaw.name }}
        </h2>
        <p
          class="description"
          :style="{ color: configStore.getColor('text') }"
        >
          {{ selectedLaw.description }}
        </p>
        <h3
          class="example-heading"
          :style="{ color: configStore.getColor('text') }"
        >
          EXEMPLO:
        </h3>
        <section
          class="example-card"
          :style="{ backgroundColor: configStore.getColor('board') }"
          :aria-label="`Exemplos de ${selectedLaw.name}`"
        >
          <p
            v-for="example in selectedLaw.examples"
            :key="example"
          >
            {{ example }}
          </p>
        </section>
        <section
          class="notes"
          :style="{ color: configStore.getColor('text') }"
        >
          <h3 class="colored-text">
            Observações:
          </h3>
          <ul>
            <li
              v-for="note in selectedLaw.notes"
              :key="note"
              :style="{ color: configStore.getColor('text') }"
            >
              {{ note }}
            </li>
          </ul>
        </section>
      </article>
    </div>
  </main>
</template>

<style scoped>
@reference "../assets/main.css";

.manual-page {
  @apply min-h-dvh overflow-hidden;
}

.manual-layout {
  @apply flex min-h-[calc(100dvh-74px)] border-[1.5px] border-t-0 border-[#222];
}

.sidebar {
  @apply flex w-[231px] flex-[0_0_231px] flex-col border-r-[1.5px] border-[#222];
}

.sidebar h1 {
  @apply m-0 border-b-[1.5px] border-[#222] px-3 pt-[23px] pb-[21px] text-center font-roboto text-[19px] font-extrabold uppercase leading-[1.28];
}

.sidebar nav {
  @apply flex flex-col;
}

.law-link {
  @apply min-h-[46px] cursor-pointer border-0 border-b-[1.5px] border-[#222] bg-transparent px-[11px] text-left font-roboto text-[25px] font-normal leading-none tracking-[.2px] text-[#282d2d];
}

.law-link:hover,
.law-link:focus-visible {
  @apply bg-[#c9f0e9] outline-none;
}

.topic {
  @apply mt-auto flex flex-col text-[19px] tracking-[.2px];
}

.topic span {
  @apply px-[11px] pt-0 pb-px;
}

.topic a {
  @apply border-t-[1.5px] border-[#222] px-[23px] py-[10px] no-underline;
}

.topic a:hover {
  @apply bg-[#0d8186];
}

.content {
  @apply min-w-0 flex-1 pt-[14px] pr-[22px] pb-[46px] pl-[22px];
}

.content h2 {
  @apply m-0 text-[61px] font-black leading-[1.12] tracking-[.2px];
}

.description {
  @apply mt-[13px] mr-0 mb-[18px] ml-0 max-w-[940px] text-[20px] leading-[1.28] tracking-[.4px];
}

.example-heading {
  @apply mt-0 mr-0 mb-1 ml-0 text-[24px] font-black leading-[1.1];
}

.example-card {
  @apply box-border min-h-[314px] rounded-[19px] border-[1.5px] border-[#222] px-[35px] py-[26px];
}

.example-card p {
  @apply m-0 text-[60px] font-black leading-[1.22] tracking-[.5px];
}

.example-card p+p {
  @apply mt-2;
}

.notes {
  @apply mt-[11px];
}

.notes h3 {
  @apply mt-0 mr-0 mb-[10px] ml-0 text-[29px] font-black leading-[1.1];
}

.notes ul {
  @apply m-0 list-none p-0 text-[19px] leading-[1.35] tracking-[.35px];
}

.notes li::before {
  @apply mr-[9px];
  content: '*';
}

@media (max-width: 760px) {
  .manual-page {
    @apply overflow-auto;
  }

  .manual-layout {
    @apply flex-col;
  }

  .sidebar {
    @apply w-auto flex-none border-r-0 border-b-[1.5px] border-[#222];
  }

  .sidebar h1 {
    @apply p-4;
  }

  .sidebar nav {
    @apply grid grid-cols-2;
  }

  .law-link {
    @apply min-h-[42px] border-r-[1.5px] border-[#222] text-[18px];
  }

  .topic {
    @apply mt-3;
  }

  .content {
    @apply px-4 pt-6 pb-[35px];
  }

  .content h2 {
    @apply text-[45px];
  }

  .description {
    @apply text-[17px];
  }

  .example-card {
    @apply min-h-[220px] p-[26px];
  }

  .example-card p {
    @apply text-[39px];
  }

  .notes h3 {
    @apply text-[25px];
  }

  .notes ul {
    @apply text-[16px];
  }
}
</style>

<script setup lang="ts">
import { computed, ref } from 'vue';
import { openUrl } from '@tauri-apps/plugin-opener';
import HeaderComponent from '@/components/HeaderComponent.vue';
import { useConfigStore } from '../stores/config';
import { lawsDetails } from '../utils/shared';
import SidebarProvider from '@/components/ui/SidebarProvider.vue';
import Sidebar from '@/components/ui/Sidebar.vue';
import SidebarHeader from '@/components/ui/SidebarHeader.vue';
import SidebarContent from '@/components/ui/SidebarContent.vue';
import SidebarItem from '@/components/ui/SidebarItem.vue';
import SidebarInset from '@/components/ui/SidebarInset.vue';
import { PhCaretLeft, PhCaretRight } from '@phosphor-icons/vue';
import StatCard from '@/components/ui/StatCard.vue';
import Pagination from '@/components/ui/Pagination.vue';
import PaginationContent from '@/components/ui/PaginationContent.vue';
import PaginationItem from '@/components/ui/PaginationItem.vue';
import PaginationLink from '@/components/ui/PaginationLink.vue';
import SidebarFooter from '@/components/ui/SidebarFooter.vue';

const configStore = useConfigStore();

const selectedName = ref('Identidade');

const previousName = computed(() => {
  const index = lawsDetails.findIndex((law) => law.name === selectedName.value);
  return (index > 0) ? lawsDetails[index - 1].name : '';
});

const nextName = computed(() => {
  const index = lawsDetails.findIndex((law) => law.name === selectedName.value);
  return (index < lawsDetails.length - 1) ? lawsDetails[index + 1].name : '';
});

const selectedLaw = computed(() => lawsDetails.find((law) =>
  law.name === selectedName.value) ?? lawsDetails[0]);

const isFirstLaw = computed(() => selectedLaw.value === lawsDetails[0]);
const isLastLaw = computed(() =>
  selectedLaw.value === lawsDetails[lawsDetails.length - 1]);

async function openExternalLink(event: MouseEvent, url: string) {
  event.preventDefault();
  await openUrl(url);
};

function changeLawHelper(isNext: boolean) {
  const index = lawsDetails.findIndex((law) => law.name === selectedName.value);

  if (isNext) {
    if (index < lawsDetails.length - 1) {
      selectedName.value = lawsDetails[index + 1].name;
    }
  } else {
    if (index > 0) {
      selectedName.value = lawsDetails[index - 1].name;
    }
  }
}
</script>

<template>
  <div class="manual-page" :class="{ dark: configStore.darkTheme }">
    <HeaderComponent :is-manual-view="true" class="shrink-0" />
    <SidebarProvider class="min-h-0 flex-1">
      <Sidebar>
        <SidebarHeader class="flex justify-center items-center font-bold text-xl">
          Identidades
        </SidebarHeader>
        <SidebarContent>
          <SidebarItem
            :variant="selectedLaw == law ? 'active' : 'default'"
            v-for="law in lawsDetails" :key="law.name"
            @click="selectedName = law.name"
          >
            {{ law.name }}
          </SidebarItem>
        </SidebarContent>
        <SidebarFooter>
          <SidebarItem
            variant="active" class="hover:cursor-pointer"
            href="https://pt.wikipedia.org/wiki/%C3%81lgebra_booleana"
            @click="openExternalLink(
              $event, 'https://pt.wikipedia.org/wiki/%C3%81lgebra_booleana')"
          >
            Mais sobre o tema
          </SidebarItem>
        </SidebarFooter>
      </Sidebar>
      <SidebarInset>
        <main class="p-8">
          <h2>{{ selectedName }}</h2>
          <p>{{ selectedLaw.description }}</p>

          <StatCard
            title="Exemplo" :value="selectedLaw.examples.join('\n')"
            variant="large" color="primary"
            class="mb-3"
          />

          <Pagination>
            <PaginationContent class="flex justify-between w-full">
              <PaginationItem>
                <PaginationLink
                  size="default" is-active
                  class="hover:cursor-pointer hover:animate-[brutal-pop_0.2s_ease-out]
                    active:scale-95" :class="{'pointer-events-none opacity-50': 
                isFirstLaw }"
                  @click="changeLawHelper(false)"
                >
                  <PhCaretLeft class="h-4 w-4 stroke-3" />
                  {{ previousName }}
                </PaginationLink>
              </PaginationItem>
              <PaginationItem>
                <PaginationLink
                  size="default" is-active
                  as="button" class="hover:cursor-pointer
                  hover:animate-[brutal-pop_0.2s_ease-out] active:scale-95"
                  :class="{ 'pointer-events-none opacity-50': isLastLaw }"
                  @click="changeLawHelper(true)"
                >
                  <PhCaretRight class="h-4 w-4 stroke-3" />
                  {{ nextName }}
                </PaginationLink>
              </PaginationItem>
            </PaginationContent>
          </Pagination>

          <h2>Observações:</h2>
          <p v-for="note in selectedLaw.notes" :key="note">
            {{ note }}
          </p>
        </main>
      </SidebarInset>
    </SidebarProvider>
  </div>
</template>

<style scoped>
@reference "../assets/main.css";

.manual-page {
  @apply flex h-dvh flex-col overflow-hidden;
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

main>h2 {
  @apply mt-6 mb-2 text-lg font-bold uppercase leading-4 tracking-[.6px] text-muted-foreground;
}

main>h2:first-child {
  @apply m-0 mb-2 text-[48px] font-bold normal-case leading-[1.1] tracking-[-1.2px] text-foreground;
}

main>p {
  @apply m-0 max-w-[680px] text-[16px] leading-[1.4] text-foreground/80;
}

main>h2:first-child+p {
  @apply mb-6 max-w-[940px] text-[18px] leading-[1.5];
}

main>p+p {
  @apply mt-2;
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

  main>h2:first-child {
    @apply text-[36px];
  }

  main>h2:first-child+p {
    @apply text-[16px];
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

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
  <div class="manual-page bg-background" :class="{ dark: configStore.darkTheme }">
    <HeaderComponent :is-manual-view="true" class="shrink-0" />
    <SidebarProvider class="min-h-0 flex-1">
      <Sidebar>
        <SidebarHeader
          class="flex justify-center items-center font-bold text-xl
        text-background-inverted border-b-background-inverted!"
        >
          Identidades
        </SidebarHeader>
        <SidebarContent>
          <SidebarItem
            :variant="selectedLaw === law ? 'active' : 'default'"
            v-for="law in lawsDetails" :key="law.name"
            @click="selectedName = law.name"
            :class="selectedLaw === law ?
              'text-background-dark': 'text-background-inverted'"
          >
            {{ law.name }}
          </SidebarItem>
        </SidebarContent>
        <SidebarFooter>
          <SidebarItem
            variant="active" class="text-background-dark hover:cursor-pointer"
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
          <h2 class="text-background-inverted">
            {{ selectedName }}
          </h2>
          <p class="text-background-inverted">
            {{ selectedLaw.description }}
          </p>

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

          <h2 class="text-background-light">
            Observações:
          </h2>
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

@media (max-width: 760px) {
  .manual-page {
    @apply overflow-auto;
  }

  main>h2:first-child {
    @apply text-[36px];
  }

  main>h2:first-child+p {
    @apply text-[16px];
  }
}
</style>

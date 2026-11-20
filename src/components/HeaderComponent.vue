<script setup lang="ts">
import { useRouter } from 'vue-router';
import { useConfigStore } from '../stores/config';
import {PhBookBookmark, PhCircuitry, PhMoon} from '@phosphor-icons/vue';
import Button from './ui/Button.vue';

const props = defineProps<{
  isManualView: boolean;
}>();

const configStore = useConfigStore();
const router = useRouter();

function routerHelper(isManualView: boolean) {
  const route = isManualView ? '/': '/manual';
  router.push(route);
}
</script>

<template>
  <header class="app-header bg-(--color-primary)">
    <a
      class="brand"
      href="#"
      aria-label="Logic Reduce"
    >
      <PhCircuitry
        :size="44"
        class="text-white"
        weight="fill"
      />
    </a>

    <h1 class="text-white text-2xl font-bold">
      Logic Reduce
    </h1>

    <div class="header-actions">
      <Button
        :variant="props.isManualView ? 'accent' : 'secondary'" animation="pop"
        as="a" class="hover:cursor-pointer"
        @click="routerHelper(props.isManualView)"
      >
        <PhBookBookmark
          :size="32" color="#000000"
          weight="fill"
        />
        Manual
      </Button>
      <Button
        variant="secondary" animation="pop"
        class="hover:cursor-pointer"
        @click="configStore.darkTheme = !configStore.darkTheme"
      >
        <PhMoon
          :size="32" color="#000000"
          weight="fill"
        />
      </Button>
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

.header-actions {
  @apply ml-auto flex gap-[13px];
}

@media (max-width: 680px) {
  .app-header {
    @apply mx-[10px] mt-[12px] w-[calc(100%_-_20px)] flex-wrap items-start p-[12px];
  }

  .header-actions {
    @apply order-3 ml-auto w-full;
  }
}
</style>

<script setup lang="ts">
import { PhSquare } from '@phosphor-icons/vue';
import { useConfigStore } from '../stores/config';
import { useSimplificationStore } from '../stores/simplification';

const configStore = useConfigStore();
const simplificationStore = useSimplificationStore();
</script>

<template>
    <div class="w-10 flex flex-col items-center py-10 overflow-auto hide-scrollbar">
        <PhSquare :color="[configStore.getColor('base'), '#000000'][index % 2]" weight="fill"
            class="inline mb-5 text-xl h-12 shrink-0" v-for="(_, index) in simplificationStore.steps"
            :key="index" />
    </div>
</template>

<style scoped>
@reference "../assets/main.css";

.hide-scrollbar {
  scrollbar-width: none;
  /* Firefox */
  -ms-overflow-style: none;
  /* IE 10+ */
}

.hide-scrollbar::-webkit-scrollbar {
  display: none;
  /* Chrome, Safari, Opera */
}
</style>
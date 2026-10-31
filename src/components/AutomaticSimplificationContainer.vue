<script setup lang="ts">
import { useConfigStore } from '../stores/config.ts';
import { useSimplificationStore } from '../stores/simplification.ts';

const configStore = useConfigStore();
const simplificationStore = useSimplificationStore();

function simplificationStepHelper(index: number, expression: string) {
  return `${simplificationStore.stepsAutomatic[index - 1].expression} -> ${expression}`;
}
</script>
<template>
  <div
    class="steps-card"
    :style="{ backgroundColor: configStore.getColor('board') }"
  >
    <article
      v-for="(step, index) in simplificationStore.stepsAutomatic"
      :key="step.title"
      class="step"
    >
      <h2>{{ index + 1 }}. <span>{{ step.title }}</span></h2>

      <p
        v-if="index == 0"
        class="step-expression"
      >
        {{ simplificationStore.submittedExpression }} -> {{ step.expression }}
      </p>
      <p
        v-else
        class="step-expression"
      >
        {{ simplificationStepHelper(index, step.expression) }}
      </p>

      <p class="step-description">
        {{ step.description }}
      </p>
    </article>
  </div>
</template>

<style scoped>
@reference "../assets/main.css";

.steps-card {
  @apply mt-[7px] py-[27px] max-[760px]:py-[23px];
}

.step + .step {
  @apply mt-8;
}

.step h2 {
  @apply mb-[6px] text-4xl leading-[1.25] max-[760px]:text-[22px];
}

.step-expression,
.step-description {
  @apply mb-4 text-3xl leading-[1.45] max-[760px]:text-[14px];
  @apply max-[760px]:[overflow-wrap:anywhere];
}

.step-expression {
  @apply font-inter;
}

.step-description {
  @apply ml-3;
}
</style>

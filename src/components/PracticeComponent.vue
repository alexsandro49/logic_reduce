<script setup lang="ts">
 
 
/* eslint-disable vue/valid-v-for */

import { PhTrash } from '@phosphor-icons/vue';
import Button from './ui/Button.vue';
import Input from './ui/Input.vue';
import Select from './ui/Select.vue';
import SelectContent from './ui/SelectContent.vue';
import SelectItem from './ui/SelectItem.vue';
import SelectTrigger from './ui/SelectTrigger.vue';
import SelectValue from './ui/SelectValue.vue';
import Timeline from './ui/Timeline.vue';
import TimelineConnector from './ui/TimelineConnector.vue';
import TimelineContent from './ui/TimelineContent.vue';
import TimelineDescription from './ui/TimelineDescription.vue';
import TimelineDot from './ui/TimelineDot.vue';
import TimelineItem from './ui/TimelineItem.vue';
import ScrollArea from './ui/ScrollArea.vue';
import { Laws, lawsSelect } from '@/utils/shared.ts';
import { useSimplificationStore } from '@/stores/simplification.ts';

const props = defineProps<{
  verificationResults: boolean[];
}>();

const simplificationStore = useSimplificationStore();

function addStep() {
  simplificationStore.steps.push({value: '', law: Laws.Final});
}

function removeStep(index: number) {
  simplificationStore.steps.splice(index, 1);
}

function selectColorBackgroundHelper(index: number) {
  if (props.verificationResults[index] === undefined) {
    return 'bg-none';
  } else if (props.verificationResults[index] === true) {
    return 'bg-secondary text-black';
  } else {
    return 'bg-destructive';
  }
}
</script>

<template>
  <ScrollArea class="flex-1 min-h-0 pe-4">
    <Timeline>
      <TimelineItem
        status="completed" v-for="(step, index) in simplificationStore.steps"
        :key="index"
      >
        <TimelineDot
          status="current" class="text-white font-bold"
          :class="selectColorBackgroundHelper(index)"
        >
          {{ index + 1 }}
        </TimelineDot>
        <TimelineConnector status="completed" />
        <TimelineContent>
          <TimelineDescription class="text-lg flex gap-3">
            <Input
              placeholder="Expressão após esse passo" class="min-w-0 flex-1 font-bold"
              v-model="step.value"
            />

            <Select v-model="step.law">
              <SelectTrigger class="w-44 shrink-0">
                <SelectValue placeholder="Identidades" class="min-w-0 truncate" />
              </SelectTrigger>
              <SelectContent>
                <SelectItem v-for="law in lawsSelect" :value="law.value">
                  {{ law.text }}
                </SelectItem>
              </SelectContent>
            </Select>

            <Button
              class="bg-destructive" @click="removeStep(index)"
              animation="pop"
            >
              <PhTrash :size="32" />
            </Button>
          </TimelineDescription>
        </TimelineContent>
      </TimelineItem>

      <TimelineItem status="completed">
        <TimelineContent>
          <Button
            class="bg-secondary text-black" @click="addStep"
            animation="pop"
          >
            <PhTrash :size="32" weight="fill" />
            <p>Adicionar</p>
          </Button>
        </TimelineContent>
      </TimelineItem>
    </Timeline>
  </ScrollArea>
</template>

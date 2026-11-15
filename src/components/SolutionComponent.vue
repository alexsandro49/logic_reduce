<script setup lang="ts">
import ScrollArea from './ui/ScrollArea.vue';
import Timeline from './ui/Timeline.vue';
import TimelineCard from './ui/TimelineCard.vue';
import TimelineConnector from './ui/TimelineConnector.vue';
import TimelineContent from './ui/TimelineContent.vue';
import TimelineDescription from './ui/TimelineDescription.vue';
import TimelineDot from './ui/TimelineDot.vue';
import TimelineItem from './ui/TimelineItem.vue';
import TimelineTitle from './ui/TimelineTitle.vue';

const props = defineProps<{
  timelineItems: { name: string, before: string, after: string }[];
}>();
</script>

<template>
  <ScrollArea
    class="flex-1 min-h-0
              border-foreground"
  >
    <Timeline>
      <TimelineItem
        status="completed" v-for="(item, index) in props.timelineItems"
        :key="item.name"
      >
        <TimelineDot status="current" class="text-white font-bold">
          {{ index + 1 }}
        </TimelineDot>
        <TimelineConnector status="completed" />
        <TimelineContent>
          <TimelineCard>
            <TimelineTitle class="text-lg">
              {{ item.name }}
            </TimelineTitle>
            <TimelineDescription class="text-lg">
              {{ item.before }}
            </TimelineDescription>
            <TimelineDescription class="text-lg font-semibold">
              {{ item.after }}
            </TimelineDescription>
          </TimelineCard>
        </TimelineContent>
      </TimelineItem>
    </Timeline>
  </ScrollArea>
</template>
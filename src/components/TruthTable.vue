<script setup lang="ts">
import { TruthTableRows } from '@/utils/types.ts';
import Table from './ui/Table.vue';
import TableBody from './ui/TableBody.vue';
import TableCell from './ui/TableCell.vue';
import TableHead from './ui/TableHead.vue';
import TableHeader from './ui/TableHeader.vue';
import TableRow from './ui/TableRow.vue';

const props = defineProps<{
  columns: string[];
  rows: TruthTableRows[];
}>();
</script>

<template>
  <Table>
    <TableHeader class="bg-primary">
      <TableRow>
        <TableHead
          class="text-white font-bold text-center"
          v-for="(column, columnIndex) in props.columns"
          :key="columnIndex"
        >
          {{ column }}
        </TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      <TableRow
        class="odd:bg-secondary" v-for="(row, rowIndex) in props.rows"
        :key="rowIndex"
      >
        <TableCell
          class="text-center last:bg-accent last:border-l-2"
          v-for="(value, valueIndex) in row"
          :key="valueIndex"
        >
          {{ value }}
        </TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
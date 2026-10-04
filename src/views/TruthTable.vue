<script setup lang="ts">
import { ref } from 'vue'
import HeaderComponent from '../components/Header.vue'
import checkIcon from '@/assets/check-fat.svg'
import deleteIcon from '@/assets/trash.svg'
import refreshIcon from '@/assets/arrows-clockwise.svg'
import { useConfigStore } from '../stores/config.ts'

type TruthTableRow = {
  a: number; b: number; c: number; notA: number; notB: number
  notAOrB: number; notBOrC: number; conjunction: number; result: number
}

const configStore = useConfigStore();

const expression = ref('~((~A + B) & (~B + C))')
const display = ref('complete')
const displayOptions = [{ label: 'Completa', value: 'complete' }, { label: 'Reduzida', value: 'reduced' }]
const columns = [
  { key: 'a', label: 'A' }, { key: 'b', label: 'B' }, { key: 'c', label: 'C' },
  { key: 'notA', label: '~A' }, { key: 'notB', label: '~B' }, { key: 'notAOrB', label: '~A+B' },
  { key: 'notBOrC', label: '~B+C' }, { key: 'conjunction', label: '(~A+B & ~B+C)' }, { key: 'result', label: 'S' },
] as const
const rows: TruthTableRow[] = [
  { a: 0, b: 0, c: 0, notA: 1, notB: 1, notAOrB: 1, notBOrC: 1, conjunction: 1, result: 0 },
  { a: 0, b: 0, c: 1, notA: 1, notB: 1, notAOrB: 1, notBOrC: 1, conjunction: 1, result: 0 },
  { a: 0, b: 1, c: 0, notA: 1, notB: 0, notAOrB: 1, notBOrC: 0, conjunction: 0, result: 1 },
  { a: 0, b: 1, c: 1, notA: 1, notB: 0, notAOrB: 1, notBOrC: 1, conjunction: 1, result: 0 },
  { a: 1, b: 0, c: 0, notA: 0, notB: 1, notAOrB: 0, notBOrC: 1, conjunction: 0, result: 1 },
  { a: 1, b: 0, c: 1, notA: 0, notB: 1, notAOrB: 0, notBOrC: 1, conjunction: 0, result: 1 },
  { a: 1, b: 1, c: 0, notA: 0, notB: 0, notAOrB: 1, notBOrC: 0, conjunction: 0, result: 1 },
  { a: 1, b: 1, c: 1, notA: 0, notB: 0, notAOrB: 1, notBOrC: 1, conjunction: 1, result: 0 },
]

function clearExpression() { expression.value = '' }
function resetTable() { expression.value = '~((~A + B) & (~B + C))'; display.value = 'complete' }
</script>

<template>
  <main class="truth-table-page" :class="{ dark: configStore.darkTheme }">
    <HeaderComponent :active-window-button="1"/>
    <section class="truth-table-workspace" aria-label="Tabela verdade">
      <div class="table-controls">
        <label class="control-field expression-control"><span>Expressão booleana:</span><input v-model="expression" type="text" aria-label="Expressão booleana" /></label>
        <label class="control-field display-control"><span>Exibição:</span><select v-model="display" aria-label="Exibição da tabela"><option v-for="option in displayOptions" :key="option.value" :value="option.value">{{ option.label }}</option></select></label>
        <div class="control-actions" aria-label="Ações da tabela">
          <button type="button" class="action-button" aria-label="Gerar tabela"><img :src="checkIcon" alt="" /></button>
          <button type="button" class="action-button" aria-label="Limpar expressão" @click="clearExpression"><img :src="deleteIcon" alt="" /></button>
          <button type="button" class="action-button" aria-label="Restaurar valores" @click="resetTable"><img :src="refreshIcon" alt="" /></button>
        </div>
      </div>
      <section class="truth-table-card" aria-label="Resultados da tabela verdade">
        <div class="table-scroll">
            <table>
                <thead>
                    <tr>
                        <th v-for="column in columns" :key="column.key" scope="col">{{ column.label }}</th>
                    </tr>
                </thead>
                <tbody>
                    <tr v-for="(row, index) in rows" :key="`${row.a}-${row.b}-${row.c}`" :class="{ highlighted: index % 2 === 0 }">
                        <td v-for="column in columns" :key="column.key">{{ row[column.key] }}</td>
                    </tr>
                </tbody>
            </table>
        </div>
      </section>
    </section>
  </main>
</template>

<style scoped>
@reference "../assets/main.css";

:global(html), :global(body), :global(#app) { @apply min-h-full; }
.truth-table-page { @apply box-border flex min-h-dvh flex-col m-0 bg-white p-0 font-roboto text-[#282d2d]; }
.truth-table-workspace { @apply box-border flex min-h-0 flex-1 flex-col border-[1.5px] border-t-0 border-[#222] bg-white px-[30px] pt-[31px] pb-[28px] dark:bg-gunmetal; }
.table-controls, .control-actions { @apply flex items-end gap-[13px]; }
.control-field { @apply flex flex-col gap-[6px]; }
.control-field > span { @apply text-[19px] font-extrabold uppercase tracking-[.1px] text-[#303434] dark:text-light-cyan; }
input, select { @apply box-border h-[49px] rounded-[11px] border-[1.5px] border-[#222] bg-light-cyan font-roboto text-[20px] text-[#202525] outline-none; }
input:focus-visible, select:focus-visible, button:focus-visible { @apply outline-[3px] outline-turquoise outline-offset-2; }
.expression-control input { @apply w-[308px] px-[13px]; }
.display-control { @apply relative; }
.display-control::after { @apply pointer-events-none absolute right-[10px] bottom-[17px] h-0 w-0 border-x-[5px] border-t-[9px] border-x-transparent border-t-[#222]; content: ''; }
.display-control select { @apply w-[150px] appearance-none py-0 pr-6 pl-2 uppercase; }
.action-button { @apply grid size-12 cursor-pointer place-items-center rounded-[11px] border-[1.5px] border-[#222] bg-turquoise; }
.action-button:hover { @apply -translate-y-px brightness-[.96]; }
.action-button img { @apply size-[27px]; }
.truth-table-card { @apply mt-[13px] box-border min-h-[768px] flex-1 overflow-hidden rounded-[23px] border-[1.5px] border-[#222] bg-light-cyan px-[30px] pt-[28px] pb-[60px] dark:bg-gunmetal dark:border-light-cyan; }
.table-scroll { @apply overflow-x-auto; }
table { @apply w-full min-w-[980px] border-separate border-spacing-0 rounded-[23px] border-[1.5px] border-[#222] text-center text-[42px] leading-none; }
th, td { @apply h-[73px] whitespace-nowrap border-b-[1.5px] border-[#222] px-3; }
th { @apply h-[72px] bg-dark-slate-grey font-medium text-[#d8efeb]; }
td { @apply font-normal text-[#303434] dark:bg-light-cyan; }
tbody tr.highlighted td { @apply bg-dark-cyan text-[#d8efeb]; }
tbody tr:last-child td { @apply border-b-0; }
th:first-child { @apply rounded-tl-[23px]; }
th:last-child { @apply rounded-tr-[23px]; }
th:nth-child(-n + 5), td:nth-child(-n + 5) { width: 8%; }
th:nth-child(6), th:nth-child(7), td:nth-child(6), td:nth-child(7) { width: 11%; }
th:nth-child(8), td:nth-child(8) { width: 25%; }
th:last-child, td:last-child { width: 9%; }

@media (max-width: 760px) {
  .truth-table-workspace { @apply px-[14px] py-6; }
  .table-controls { @apply flex-wrap items-stretch; }
  .expression-control, .expression-control input { @apply w-full; }
  .truth-table-card { @apply min-h-0 px-[14px] pt-[18px] pb-6; }
  table { @apply text-[30px]; }
}
</style>

<script setup lang="ts">
import { ref } from 'vue'
import HeaderComponent from '../components/Header.vue'
import checkIcon from '@/assets/check-fat.svg'
import deleteIcon from '@/assets/trash.svg'
import refreshIcon from '@/assets/arrows-clockwise.svg'

type TruthTableRow = {
  a: number; b: number; c: number; notA: number; notB: number
  notAOrB: number; notBOrC: number; conjunction: number; result: number
}

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
  <main class="truth-table-page">
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
                        <th v-for="column in columns" :key="column.key" scope="col"> {{ column.label }}</th>
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

:global(html), :global(body), :global(#app) { min-height: 100%; }
.truth-table-page { display: flex; box-sizing: border-box; min-height: 100dvh; flex-direction: column; margin: 0; padding: 0; background: #fff; color: #282d2d; font-family: var(--font-roboto); }
.truth-table-workspace { display: flex; box-sizing: border-box; min-height: 0; flex: 1; flex-direction: column; border: 1.5px solid #222; border-top: 0; background: #fff; padding: 31px 30px 28px; }
.table-controls, .control-actions { display: flex; align-items: end; gap: 13px; }
.control-field { display: flex; flex-direction: column; gap: 6px; }
.control-field > span { color: #303434; font-size: 19px; font-weight: 800; letter-spacing: .1px; text-transform: uppercase; }
input, select { box-sizing: border-box; height: 49px; border: 1.5px solid #222; border-radius: 11px; background: var(--color-light-cyan); color: #202525; font-family: var(--font-roboto); font-size: 20px; outline: none; }
input:focus-visible, select:focus-visible, button:focus-visible { outline: 3px solid var(--color-turquoise); outline-offset: 2px; }
.expression-control input { width: 308px; padding: 0 13px; }
.display-control { position: relative; }
.display-control::after { position: absolute; right: 10px; bottom: 17px; width: 0; height: 0; border-top: 9px solid #222; border-right: 5px solid transparent; border-left: 5px solid transparent; content: ''; pointer-events: none; }
.display-control select { width: 150px; appearance: none; padding: 0 24px 0 8px; text-transform: uppercase; }
.action-button { display: grid; width: 48px; height: 48px; place-items: center; border: 1.5px solid #222; border-radius: 11px; background: var(--color-turquoise); cursor: pointer; }
.action-button:hover { filter: brightness(.96); transform: translateY(-1px); }
.action-button img { width: 27px; height: 27px; }
.truth-table-card { box-sizing: border-box; min-height: 768px; flex: 1; margin-top: 13px; padding: 28px 30px 60px; overflow: hidden; border: 1.5px solid #222; border-radius: 23px; background: var(--color-light-cyan); }
.table-scroll { overflow-x: auto; }
table { width: 100%; min-width: 980px; border: 1.5px solid #222; border-collapse: separate; border-spacing: 0; border-radius: 23px; font-size: 42px; line-height: 1; text-align: center; }
th, td { height: 73px; padding: 0 12px; border-bottom: 1.5px solid #222; white-space: nowrap; }
th { height: 72px; background: var(--color-dark-slate-grey); color: #d8efeb; font-weight: 500; }
td { color: #303434; font-weight: 400; }
tbody tr.highlighted td { background: var(--color-dark-cyan); color: #d8efeb; }
tbody tr:last-child td { border-bottom: 0; }
th:nth-child(1) { border-radius: 23px 0px 0px 0px;}
th:last-child { border-radius: 0px 23px 0px 0px;}
th:nth-child(-n + 5), td:nth-child(-n + 5) { width: 8%; }
th:nth-child(6), th:nth-child(7), td:nth-child(6), td:nth-child(7) { width: 11%; }
th:nth-child(8), td:nth-child(8) { width: 25%; }
th:last-child, td:last-child { width: 9%; }
@media (max-width: 760px) { .truth-table-workspace { padding: 24px 14px; } .table-controls { align-items: stretch; flex-wrap: wrap; } .expression-control { width: 100%; } .expression-control input { width: 100%; } .truth-table-card { min-height: 0; padding: 18px 14px 24px; } table { font-size: 30px; } }
</style>

<script setup lang="ts">
import { computed, ref } from 'vue'
import { openUrl } from '@tauri-apps/plugin-opener';
import HeaderComponent from '@/components/Header.vue'
import { useConfigStore } from '../stores/config';

type Rule = { name: string; description: string; examples: string[]; notes: string[] }

const configStore = useConfigStore();

const rules: Rule[] = [
  { name: 'Identidade', description: 'Operar com o elemento neutro não altera o valor.', examples: ['A + 0 ⇒ A', 'A & 1 ⇒ A'], notes: ['Útil para remover constantes que não alteram o resultado da expressão.'] },
  { name: 'Dominação', description: 'O valor dominante determina o resultado da operação.', examples: ['A + 1 ⇒ 1', 'A & 0 ⇒ 0'], notes: ['O termo dominante depende do operador utilizado.', 'O valor dominante torna os demais termos da operação irrelevantes.'] },
  { name: 'Idempotência', description: 'A repetição de um termo não muda o resultado da expressão.', examples: ['A + A ⇒ A', 'A & A ⇒ A'], notes: ['Termos repetidos podem ser reduzidos a uma única ocorrência.'] },
  { name: 'Complemento', description: 'Uma variável combinada com seu complemento produz sempre 1 no OU e 0 no E.', examples: ['A + ~A ⇒ 1', 'A & ~A ⇒ 0'], notes: ['A negação é representada pelo símbolo ~.', 'Uma variável e sua negação sempre possuem valores opostos.'] },
  { name: 'D. Negação', description: 'Duas negações consecutivas se anulam.', examples: ['~(~A) ⇒ A'], notes: ['A expressão retorna ao seu valor original.'] },
  { name: 'Comutativa', description: 'A ordem dos termos não altera o resultado.', examples: ['A + B ⇒ B + A', 'A & B ⇒ B & A'], notes: ['Pode ser usada para reorganizar os termos e facilitar a aplicação de outras regras.'] },
  { name: 'Associativa', description: 'O agrupamento das operações não altera o resultado quando a operação é a mesma.', examples: ['(A + B) + C ⇒ A + (B + C)', '(A & B) & C ⇒ A & (B & C)'], notes: ['Permite reagrupar termos de uma mesma operação sem alterar o resultado.'] },
  { name: 'Distributiva', description: 'Permite distribuir uma operação sobre outra para reorganizar ou simplificar a expressão.', examples: ['A & (B + C) ⇒ (A & B) + (A & C)'], notes: ['Permite distribuir uma operação sobre outra para reorganizar ou simplificar a expressão.'] },
  { name: 'Absorção', description: 'Um termo mais simples pode absorver outro termo que já depende dele.', examples: ['A + (A & B) ⇒ A', 'A & (A + B) ⇒ A'], notes: ['A variável comum determina o resultado.', 'Remove termos redundantes que não influenciam o resultado final.'] },
  { name: 'De Morgan', description: 'Ao negar uma expressão, troca o operador e nega cada termo.', examples: ['~(A + B) ⇒ ~A & ~B', '~(A & B) ⇒ ~A + ~B'], notes: ['A lei ajuda a mover negações para dentro de parênteses.'] },
]
const selectedName = ref('Identidade')
const selectedRule = computed(() => rules.find((rule) => rule.name === selectedName.value) ?? rules[0])

async function openExternalLink(event: MouseEvent, url: string) {
  event.preventDefault();
  await openUrl(url);
};
</script>

<template>
  <main class="manual-page" :class="{dark: configStore.darkTheme}">
    <HeaderComponent :active-window-button="2" />
    <div class="manual-layout">
      <aside class="sidebar" aria-label="Regras da álgebra booleana">
        <h1>Propriedades</h1>
        <nav>
          <button v-for="rule in rules" :key="rule.name" class="rule-link" :class="{ active: selectedName === rule.name }" :aria-current="selectedName === rule.name ? 'page' : undefined" @click="selectedName = rule.name">{{ rule.name }}</button>
        </nav>
        <div class="topic"><span>Sobre o tema:</span><a href="https://pt.wikipedia.org/wiki/%C3%81lgebra_booleana" @click="openExternalLink($event, 'https://pt.wikipedia.org/wiki/%C3%81lgebra_booleana')">Álgebra Booleana</a></div>
      </aside>

      <article class="content">
        <h2 class="colored-text">{{ selectedRule.name === 'D. Negação' ? 'Dupla Negação' : selectedRule.name }}</h2>
        <p class="description colored-text">{{ selectedRule.description }}</p>
        <h3 class="example-heading colored-text">EXEMPLO:</h3>
        <section class="example-card" :aria-label="`Exemplos de ${selectedRule.name}`"><p v-for="example in selectedRule.examples" :key="example">{{ example }}</p></section>
        <section class="notes"><h3 class="colored-text">Observações:</h3><ul><li v-for="note in selectedRule.notes" :key="note" :class="{'text-light-cyan': configStore.darkTheme}">{{ note }}</li></ul></section>
      </article>
    </div>
  </main>
</template>

<style scoped>
@reference "../assets/main.css";
:global(html), :global(body), :global(#app) { @apply min-h-full; }
.manual-page { @apply min-h-dvh overflow-hidden bg-white font-roboto text-[#242424]; }
.manual-layout { @apply flex min-h-[calc(100dvh-74px)] border-[1.5px] border-t-0 border-[#222] dark:bg-gunmetal; }
.sidebar { @apply flex w-[231px] flex-[0_0_231px] flex-col border-r-[1.5px] border-[#222] bg-light-cyan; }
.sidebar h1 { @apply m-0 border-b-[1.5px] border-[#222] px-3 pt-[23px] pb-[21px] text-center font-roboto text-[19px] font-extrabold uppercase leading-[1.28]; }
.sidebar nav { @apply flex flex-col; }
.rule-link { @apply min-h-[46px] cursor-pointer border-0 border-b-[1.5px] border-[#222] bg-transparent px-[11px] text-left font-roboto text-[25px] font-normal leading-none tracking-[.2px] text-[#282d2d]; }
.rule-link:hover, .rule-link:focus-visible { @apply bg-[#c9f0e9] outline-none; }
.rule-link.active { @apply bg-turquoise; }
.topic { @apply mt-auto flex flex-col text-[19px] tracking-[.2px]; }
.topic span { @apply px-[11px] pt-0 pb-px; }
.topic a { @apply border-t-[1.5px] border-[#222] bg-dark-cyan px-[23px] py-[10px] text-light-cyan no-underline; }
.topic a:hover { @apply bg-[#0d8186]; }
.content { @apply min-w-0 flex-1 pt-[14px] pr-[22px] pb-[46px] pl-[22px]; }
.content h2 { @apply m-0 text-[61px] font-black leading-[1.12] tracking-[.2px]; }
.description { @apply mt-[13px] mr-0 mb-[18px] ml-0 max-w-[940px] text-[20px] leading-[1.28] tracking-[.4px]; }
.example-heading { @apply mt-0 mr-0 mb-1 ml-0 text-[24px] font-black leading-[1.1]; }
.example-card { @apply box-border min-h-[314px] rounded-[19px] border-[1.5px] border-[#222] bg-light-cyan px-[35px] py-[26px]; }
.example-card p { @apply m-0 text-[60px] font-black leading-[1.22] tracking-[.5px]; }
.example-card p + p { @apply mt-2; }
.notes { @apply mt-[11px]; }
.notes h3 { @apply mt-0 mr-0 mb-[10px] ml-0 text-[29px] font-black leading-[1.1]; }
.notes ul { @apply m-0 list-none p-0 text-[19px] leading-[1.35] tracking-[.35px]; }
.notes li::before { @apply mr-[9px]; content: '*'; }
.colored-text { @apply dark:text-light-cyan; }

@media (max-width: 760px) {
  .manual-page { @apply overflow-auto; }
  .manual-layout { @apply flex-col; }
  .sidebar { @apply w-auto flex-none border-r-0 border-b-[1.5px] border-[#222]; }
  .sidebar h1 { @apply p-4; }
  .sidebar nav { @apply grid grid-cols-2; }
  .rule-link { @apply min-h-[42px] border-r-[1.5px] border-[#222] text-[18px]; }
  .topic { @apply mt-3; }
  .content { @apply px-4 pt-6 pb-[35px]; }
  .content h2 { @apply text-[45px]; }
  .description { @apply text-[17px]; }
  .example-card { @apply min-h-[220px] p-[26px]; }
  .example-card p { @apply text-[39px]; }
  .notes h3 { @apply text-[25px]; }
  .notes ul { @apply text-[16px]; }
}
</style>

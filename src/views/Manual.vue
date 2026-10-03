<script setup lang="ts">
import { computed, ref } from 'vue'
import HeaderComponent from '@/components/Header.vue'

type Rule = { name: string; description: string; examples: string[]; notes: string[] }
const rules: Rule[] = [
  { name: 'Identidade', description: 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Maecenas lobortis turpis non est pulvinar, ut facilisis sapien ultrices. Cras vitae mattis quam.', examples: ['A + 0 ⇒ A', 'A & 1 ⇒ A'], notes: ['Lorem ipsum dolor sit amet, consectetur adipiscing elit.', 'Maecenas lobortis turpis non est pulvinar, ut facilisis sapien ultrices.'] },
  { name: 'Dominação', description: 'Valores dominantes definem o resultado de uma operação booleana.', examples: ['A + 1 ⇒ 1', 'A & 0 ⇒ 0'], notes: ['O termo dominante depende do operador utilizado.'] },
  { name: 'Idempotência', description: 'A repetição de um termo não muda o resultado da expressão.', examples: ['A + A ⇒ A', 'A & A ⇒ A'], notes: ['Termos repetidos podem ser removidos.'] },
  { name: 'Complemento', description: 'Uma variável e sua negação formam um complemento.', examples: ['A + ~A ⇒ 1', 'A & ~A ⇒ 0'], notes: ['A negação é representada pelo símbolo ~.'] },
  { name: 'D. Negação', description: 'Duas negações consecutivas se anulam.', examples: ['~(~A) ⇒ A'], notes: ['A expressão retorna ao seu valor original.'] },
  { name: 'Comutativa', description: 'A ordem dos termos não altera o resultado.', examples: ['A + B ⇒ B + A', 'A & B ⇒ B & A'], notes: ['Reorganize os termos quando necessário.'] },
  { name: 'Associativa', description: 'O agrupamento das operações não altera a expressão.', examples: ['(A + B) + C ⇒ A + (B + C)', '(A & B) & C ⇒ A & (B & C)'], notes: ['A regra se aplica a operadores iguais.'] },
  { name: 'Distributiva', description: 'Permite expandir ou fatorar uma expressão.', examples: ['A & (B + C) ⇒ (A & B) + (A & C)'], notes: ['Use-a para expor novas simplificações.'] },
  { name: 'Absorção', description: 'Um termo pode absorver outro mais específico.', examples: ['A + (A & B) ⇒ A', 'A & (A + B) ⇒ A'], notes: ['A variável comum determina o resultado.'] },
  { name: 'De Morgan', description: 'A negação troca o operador e nega cada termo.', examples: ['~(A + B) ⇒ ~A & ~B', '~(A & B) ⇒ ~A + ~B'], notes: ['A lei ajuda a mover negações para dentro de parênteses.'] },
]
const selectedName = ref('Identidade')
const selectedRule = computed(() => rules.find((rule) => rule.name === selectedName.value) ?? rules[0])
</script>

<template>
  <main class="manual-page">
    <HeaderComponent :active-window-button="2" />
    <div class="manual-layout">
      <aside class="sidebar" aria-label="Regras da álgebra booleana">
        <h1>Regras Álgebra<br>Booleana</h1>
        <nav>
          <button v-for="rule in rules" :key="rule.name" class="rule-link" :class="{ active: selectedName === rule.name }" :aria-current="selectedName === rule.name ? 'page' : undefined" @click="selectedName = rule.name">{{ rule.name }}</button>
        </nav>
        <div class="topic"><span>Sobre o tema:</span><a href="https://pt.wikipedia.org/wiki/%C3%81lgebra_booleana" target="_blank" rel="noopener">Álgebra Booleana</a></div>
      </aside>

      <article class="content">
        <h2>{{ selectedRule.name === 'D. Negação' ? 'Dupla Negação' : selectedRule.name }}</h2>
        <p class="description">{{ selectedRule.description }}</p>
        <h3 class="example-heading">EXEMPLO:</h3>
        <section class="example-card" :aria-label="`Exemplos de ${selectedRule.name}`"><p v-for="example in selectedRule.examples" :key="example">{{ example }}</p></section>
        <section class="notes"><h3>Observações:</h3><ul><li v-for="note in selectedRule.notes" :key="note">{{ note }}</li></ul></section>
      </article>
    </div>
  </main>
</template>

<style scoped>
@reference "../assets/main.css";
:global(html), :global(body), :global(#app) { min-height: 100%; }
.manual-page { min-height: 100dvh; overflow: hidden; background: white; color: #242424; font-family: var(--font-roboto); }
.manual-page :deep(.app-header) { gap: 11px; min-height: 59px; padding: 8px 12px 8px 14px; }
.manual-page :deep(.brand) { flex-basis: 32px; height: 32px; width: 32px; }
.manual-page :deep(.brand img) { height: 38px; width: 38px; }
.manual-page :deep(.main-navigation) { gap: 10px; }
.manual-page :deep(.navigation-link) { min-height: 36px; padding: 0 9px; font-size: 14px; }
.manual-page :deep(.header-actions) { gap: 10px; }
.manual-page :deep(.icon-button) { height: 36px; width: 36px; }
.manual-page :deep(.icon-button img) { height: 22px; width: 22px; }
.manual-layout { display: flex; min-height: calc(100dvh - 59px); border: 1.5px solid #222; border-top: 0; }
.sidebar { display: flex; width: 231px; flex: 0 0 231px; flex-direction: column; border-right: 1.5px solid #222; background: var(--color-light-cyan); }
.sidebar h1 { margin: 0; padding: 23px 12px 21px; border-bottom: 1.5px solid #222; font-size: 19px; font-weight: 800; line-height: 1.28; text-align: center; }
.sidebar nav { display: flex; flex-direction: column; }
.rule-link { min-height: 46px; border: 0; border-bottom: 1.5px solid #222; background: transparent; color: #282d2d; cursor: pointer; font: 400 25px/1 var(--font-roboto); letter-spacing: .2px; padding: 0 11px; text-align: left; }
.rule-link:hover, .rule-link:focus-visible { background: #c9f0e9; outline: none; }
.rule-link.active { background: var(--color-turquoise); }
.topic { display: flex; flex-direction: column; margin-top: auto; font-size: 19px; letter-spacing: .2px; }
.topic span { padding: 0 11px 1px; }.topic a { border-top: 1.5px solid #222; background: var(--color-dark-cyan); color: #e4f9f5; padding: 10px 23px; text-decoration: none; }.topic a:hover { background: #0d8186; }
.content { min-width: 0; flex: 1; padding: 14px 22px 46px; }.content h2 { margin: 0; font-size: 61px; font-weight: 900; letter-spacing: .2px; line-height: 1.12; }.description { max-width: 940px; margin: 13px 0 18px; font-size: 20px; letter-spacing: .4px; line-height: 1.28; }
.example-heading { margin: 0 0 4px; font-size: 24px; font-weight: 900; line-height: 1.1; }
.example-card { box-sizing: border-box; min-height: 314px; border: 1.5px solid #222; border-radius: 19px; background: var(--color-light-cyan); padding: 26px 35px; }.example-card p { margin: 0; font-size: 60px; font-weight: 900; letter-spacing: .5px; line-height: 1.22; }.example-card p + p { margin-top: 8px; }
.notes { margin-top: 11px; }.notes h3 { margin: 0 0 10px; font-size: 29px; font-weight: 900; line-height: 1.1; }.notes ul { margin: 0; padding: 0; list-style: none; font-size: 19px; letter-spacing: .35px; line-height: 1.35; }.notes li::before { content: '*'; margin-right: 9px; }
@media (max-width: 760px) { .manual-page { overflow: auto; }.manual-layout { flex-direction: column; }.sidebar { width: auto; flex: none; border-right: 0; border-bottom: 1.5px solid #222; }.sidebar h1 { padding: 16px; }.sidebar nav { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); }.rule-link { min-height: 42px; border-right: 1.5px solid #222; font-size: 18px; }.topic { margin-top: 12px; }.content { padding: 24px 16px 35px; }.content h2 { font-size: 45px; }.description { font-size: 17px; }.example-card { min-height: 220px; padding: 26px; }.example-card p { font-size: 39px; }.notes h3 { font-size: 25px; }.notes ul { font-size: 16px; } }
</style>

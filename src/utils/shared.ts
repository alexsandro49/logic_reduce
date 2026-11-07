import { LawDetails } from './types';

export const lawsSelect = [
  { text: 'IDENTIDADE', value: 'identity' },
  { text: 'DOMINAÇÃO', value: 'null' },
  { text: 'IDEMPOTÊNCIA', value: 'idempotent' },
  { text: 'COMPLEMENTO', value: 'complement' },
  { text: 'COMUTATIVA', value: 'commutative' },
  { text: 'ASSOCIATIVA', value: 'associative' },
  { text: 'DISTRIBUTIVA', value: 'distributive' },
  { text: 'ABSORÇÃO', value: 'absorption' },
  { text: 'D. MORGAN', value: 'de_morgan' },
  { text: 'D. NEGAÇÃO', value: 'double_negative' },
  { text: 'FINAL', value: 'final' }
];

export const lawsDetails: LawDetails[] = [
  {
    name: 'Identidade', description: 'Operar com o elemento neutro não altera o valor.',
    examples: ['A + 0 ⇒ A', 'A & 1 ⇒ A'], notes: ['Útil para remover constantes que não\
     alteram o resultado da expressão.']
  },
  {
    name: 'Dominação', description: 'O valor dominante determina o resultado da\
   operação.', examples: ['A + 1 ⇒ 1', 'A & 0 ⇒ 0'], notes: ['O termo dominante depende\
   do operador utilizado.', 'O valor dominante torna os demais termos da operação\
   irrelevantes.']
  },
  {
    name: 'Idempotência', description: 'A repetição de um termo não muda o resultado da\
  expressão.', examples: ['A + A ⇒ A', 'A & A ⇒ A'], notes: ['Termos repetidos podem ser\
  reduzidos a uma única ocorrência.']
  },
  {
    name: 'Complemento', description: 'Uma variável combinada com seu complemento produz\
  sempre 1 no OU e 0 no E.', examples: ['A + ~A ⇒ 1', 'A & ~A ⇒ 0'], notes: ['A negação é\
  representada pelo símbolo ~.',
    'Uma variável e sua negação sempre possuem valores opostos.']
  },
  {
    name: 'Comutativa', description: 'A ordem dos termos não altera o resultado.',
    examples: ['A + B ⇒ B + A', 'A & B ⇒ B & A'], notes: ['Pode ser usada para\
    reorganizar os termos e facilitar a aplicação de outras regras.']
  },
  {
    name: 'Associativa', description: 'O agrupamento das operações não altera o resultado\
   quando a operação é a mesma.', examples: ['(A + B) + C ⇒ A + (B + C)',
      '(A & B) & C ⇒ A & (B & C)'], notes: ['Permite reagrupar termos de uma mesma\
    operação sem alterar o resultado.']
  },
  {
    name: 'Distributiva', description: 'Permite distribuir uma operação sobre outra para\
   reorganizar ou simplificar a expressão.',
    examples: ['A & (B + C) ⇒ (A & B) + (A & C)'], notes: ['Permite distribuir uma\
   operação sobre outra para reorganizar ou simplificar a expressão.']
  },
  {
    name: 'Absorção', description: 'Um termo mais simples pode absorver outro termo que\
   já depende dele.', examples: ['A + (A & B) ⇒ A', 'A & (A + B) ⇒ A'],
    notes: ['A variável comum determina o resultado.', 'Remove termos redundantes que não\
   influenciam o resultado final.']
  },
  {
    name: 'De Morgan', description: 'Ao negar uma expressão, troca o operador e nega\
  cada termo.', examples: ['~(A + B) ⇒ ~A & ~B', '~(A & B) ⇒ ~A + ~B'],
    notes: ['A lei ajuda a mover negações para dentro de parênteses.']
  },
  {
    name: 'D. Negação', description: 'Duas negações consecutivas se anulam.',
    examples: ['~(~A) ⇒ A'], notes: ['A expressão retorna ao seu valor original.']
  },
];
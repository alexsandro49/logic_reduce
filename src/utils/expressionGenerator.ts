import { Ref } from "vue";

const variables = ['A', 'B', 'C', 'D'];
const binaryOperators = ['&', '+', '^'];

function randomItem<T>(items: readonly T[]): T {
  return items[Math.floor(Math.random() * items.length)];
}

function randomOperand(depth: number): string {
  if (depth === 0 || Math.random() < 0.25) {
    return Math.random() < 0.15
      ? (Math.random() < 0.5 ? '0' : '1')
      : randomItem(variables);
  }

  if (Math.random() < 0.25) {
    return `~${randomOperand(depth - 1)}`;
  }

  const left = randomOperand(depth - 1);
  const operator = randomItem(binaryOperators);
  const right = randomOperand(depth - 1);
  return `(${left} ${operator} ${right})`;
}

export function generateRandomExpression(expression: Ref<string>) {
  const depth = Math.random() < 0.5 ? 2 : 3;
  const left = randomOperand(depth - 1);
  const operator = randomItem(binaryOperators);
  const right = randomOperand(depth - 1);

  expression.value = `(${left} ${operator} ${right})`;

  return expression.value
}
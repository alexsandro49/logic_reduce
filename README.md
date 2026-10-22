# 💻 Logic Reduce

**Logic Reduce** é um simplificador de Álgebra Booleana, desenvolvido com Tauri e Vue. Auxiliando a simplificação, visualização passo a passo, tabela verdade, e exemplos didáticos.

---

## 📸 Preview

![Imagem do projeto em execução](https://github.com/alexsandro49/logic_reduce/blob/main/screenshot.png)

---

## 🚀 Tecnologias utilizadas
### Front-end:
- [TypeScript](https://www.typescriptlang.org/)
- [Vue.js](https://vuejs.org/)
- [Pinia](https://pinia.vuejs.org/)
- [TailwindCSS](https://tailwindcss.com/)
- [Vite](https://vitejs.dev/)
### Back-end:
- [Rust](https://rust-lang.org/)
- [Tauri](https://v2.tauri.app/)

---

## 🧱 Funcionalidades

- Listagem de sabores de pizza
- Filtro por tipo (tradicional, especial, doce, etc)
- Exibição de preços e descrições
- Carrinho de compras
- Design responsivo

---

## 📁 Estrutura de Pastas
```
src/
├── assets/         # Imagens e ícones
├── components/     # Componentes reutilizáveis
├── routes/         # Configuração das rotas (Vue Router)
├── stores/         # Pinia stores
├── types/          # Tipagens TypeScript
├── utils/          # Funções utilitárias
├── App.vue         # Componente raiz
├── main.ts         # Inicialização do app
├── src-tauri/      # Projeto Rust
```

## Como executar o projeto:
   ```
   pnpm tauri dev
   ```

## Referências
- [Phosphor Icons](https://phosphoricons.com/)

## License
- [GPLv3](https://github.com/alexsandro49/logic_reduce/blob/main/LICENSE)

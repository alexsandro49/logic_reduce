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

- Auxílio do aprendizado de Álgebra Booleana
- Simplificação automática
- Tabela verdade
- Exemplos didáticos

---

## 📁 Estrutura de Pastas
```
src/                # Projeto Typescript
├── assets/         # Imagens e ícones
├── components/     # Componentes reutilizáveis
├── routes/         # Configuração das rotas (Vue Router)
├── stores/         # Pinia stores
├── types/          # Tipagens TypeScript
├── utils/          # Funções utilitárias
├── App.vue         # Componente raiz
├── main.ts         # Inicialização do app
src-tauri/          # Projeto Rust
├── capabilities/   # Arquivos de configuração
├── icons/          # Ícones do aplicativo
├── src/            # Código-fonte do backend
├──── commands/     # Funções que o frontend pode chamar via IPC
├──── models/       # Estruturas de dados que representam as entidades
├──── services/     # Lógica de negócios
├──── lib.rs        # Usado para configurar o construtor do Tauri e registrar plugins
├──── main.rs       # Ponto de entrada principal do executável
├── build.rs        # Script de compilação executado antes da construção do projeto
```

## Como executar o projeto:
   ```
   pnpm tauri dev
   ```

## Referências
- [Phosphor Icons](https://phosphoricons.com/)

## License
- [GPLv3](https://github.com/alexsandro49/logic_reduce/blob/main/LICENSE)

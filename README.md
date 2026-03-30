# GBA-emulator

## Descrição
Projeto de emulação do console Game Boy Advance (GBA) desenvolvido em dupla. O objetivo central é o estudo aprofundado de arquitetura de computadores, sistemas embarcados e programação de sistemas utilizando Rust, explorando a implementação de hardware via software (emulação).

## Status
Pausado

## Frentes de Estudo e Implementação
O projeto foi interrompido na fase inicial de pesquisa e estruturação dos componentes core da arquitetura ARM7TDMI. O código existente reflete esse estágio: estruturas básicas de CPU e memória implementadas, sem execução de instruções ou loop de emulação.

- **Mapeamento de Memória (Memory Mapping)** — estudo e implementação inicial das regiões ROM, EWRAM e IWRAM com endereçamento correto.
- **Ciclo de Instruções (CPU)** — pesquisa e estruturação do fetch e classificação de instruções ARM e THUMB; execução não implementada.
- **Gerenciamento de Recursos em Rust** — exploração de *ownership* e *borrow checker* para garantir performance e segurança de memória na emulação.

## Tecnologias
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

## Observações
- **Escolha do Rust** — decidida pela necessidade de controle de memória de baixo nível sem abrir mão de segurança (memory safety), essencial para a performance exigida em emulação.
- **Complexidade de Arquitetura** — o projeto exige a implementação fiel de componentes como PPU (gráficos), APU (áudio) e timers, o que demanda um ciclo de desenvolvimento extenso e pesquisa documental rigorosa.
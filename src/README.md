ConectaStore — Sistema de Recomendação de Produtos

1. Introdução

O ConectaStore é um sistema desenvolvido em Rust para cadastrar produtos, listar produtos, estabelecer conexões entre produtos e recomendar produtos relacionados.

O projeto utiliza estruturas de dados para organizar e relacionar os produtos cadastrados.

2. Objetivo

O objetivo é aplicar conceitos de estruturas de dados e algoritmos na construção de um sistema simples de recomendação de produtos.

3. Tecnologias utilizadas

- Linguagem de programação: Rust
- Gerenciador de projetos e compilação: Cargo
- Editor de código: Visual Studio Code

4. Estruturas de dados utilizadas

- HashMap: armazena os produtos utilizando seus IDs como chaves.
- Grafo: representa as conexões entre produtos.
- Vetores: armazenam listas de produtos relacionados.
- HashSet e VecDeque: estruturas disponíveis no código para apoiar operações de busca e organização.

5. Funcionalidades

1. Cadastrar produtos com ID, nome e categoria.
2. Listar os produtos cadastrados.
3. Conectar dois produtos no grafo.
4. Recomendar produtos relacionados a um produto de referência.
5. Validar IDs duplicados e produtos inexistentes.

6. Como executar

Abra o terminal na pasta do projeto e execute:

cargo run

Para executar os testes automatizados:

cargo test

7. Testes

Durante os testes realizados, foram aprovados dois testes automatizados relacionados ao cadastro, à listagem e à validação de IDs duplicados.

8. Conclusão

O projeto permite praticar o uso de estruturas de dados em Rust, demonstrando como produtos podem ser armazenados, conectados e recomendados por meio de um grafo.